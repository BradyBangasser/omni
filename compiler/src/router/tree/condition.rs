use base::types::http;
use im::{HashSet, Vector};
use ptree::TreeBuilder;
use std::{error::Error, sync::Arc};
use strum::IntoStaticStr;

use log::info;

use crate::router::{
    generate::{Generator, format::rs::Format, indent_fn},
    tree::{Route, pass::Pass},
};

pub trait CustomCondition: std::fmt::Debug + Send + Sync {
    fn get_type(&self) -> &'static str;
    fn gen_code(
        &self,
        writer: &mut dyn std::io::Write,
        indent: usize,
        nodes: &im::Vector<ConditionNode>,
    ) -> CodeGenRetType;
}

type CodeGenRetType = Result<usize, Box<dyn Error>>;

#[derive(Debug, Clone, IntoStaticStr)]
pub enum ConditionType {
    SegmentCount(usize),
    Length(usize),
    Prefix(String),
    Custom(Arc<dyn CustomCondition>),
}

impl PartialEq for ConditionType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SegmentCount(a), Self::SegmentCount(b)) => a == b,
            (Self::Prefix(a), Self::Prefix(b)) => a == b,
            (Self::Custom(a), Self::Custom(b)) => std::sync::Arc::ptr_eq(a, b),
            (Self::Length(a), Self::Length(b)) => a == b,
            _ => false,
        }
    }
}

impl ConditionType {
    fn _gen_prefix_code(
        writer: &mut dyn std::io::Write,
        indent: usize,
        nodes: &im::Vector<ConditionNode>,
    ) -> CodeGenRetType {
        todo!();
    }

    fn _gen_length_code<T: std::io::Write>(
        writer: &mut T,
        mut indent: usize,
        nodes: &im::Vector<ConditionNode>,
    ) -> CodeGenRetType {
        indent_fn(indent, writer)?;
        writeln!(writer, "let l = route.len();")?;
        indent_fn(indent, writer)?;
        writeln!(writer, "match l {{")?;
        indent += 1;

        for n in nodes {
            if let ConditionType::Length(l) = &n.condition {
                indent_fn(indent, writer)?;
                writeln!(writer, "{} => {{", l)?;

                Format::format_r(writer, indent + 1, &n.stored_routes, &n.children);
                indent_fn(indent, writer)?;
                writeln!(writer, "}},")?;
            }
        }

        indent_fn(indent, writer)?;
        writeln!(writer, "_ => {{}}")?;
        indent -= 1;

        indent_fn(indent, writer)?;
        writeln!(writer, "}}")?;

        Ok(indent)
    }

    fn _gen_segcount_code(
        writer: &mut dyn std::io::Write,
        indent: usize,
        nodes: &im::Vector<ConditionNode>,
    ) -> CodeGenRetType {
        todo!();
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Self::Custom(cc) => cc.get_type(),
            _ => self.into(),
        }
    }

    pub fn gen_code<T: std::io::Write>(
        &self,
        nodes: &im::Vector<ConditionNode>,
        indent: usize,
        writer: &mut T,
    ) -> CodeGenRetType {
        if nodes.is_empty() {
            return Err("No Nodes generate code for".into());
        }

        match self {
            Self::SegmentCount(_) => Self::_gen_segcount_code(writer, indent, nodes),
            Self::Prefix(_) => Self::_gen_prefix_code(writer, indent, nodes),
            Self::Length(_) => Self::_gen_length_code(writer, indent, nodes),
            Self::Custom(c) => c.gen_code(writer, indent, nodes),
            // _ => Err("Condition Undefined".into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConditionNode {
    pub condition: ConditionType,
    pub children: im::Vector<ConditionNode>,
    pub stored_routes: im::Vector<Route>,
}

#[derive(Debug)]
pub struct ConditionTree {
    children: im::Vector<ConditionNode>,
    stored_routes: im::Vector<Route>,
}

pub type PartitionFn = fn(
    pass: &mut dyn Pass,
    routes: im::Vector<Route>,
) -> im::Vector<(ConditionType, im::Vector<Route>)>;

impl ConditionTree {
    pub fn new(r: im::Vector<Route>) -> ConditionTree {
        ConditionTree {
            children: im::Vector::new(),
            stored_routes: r,
        }
    }

    pub fn get_routes(&self) -> &im::Vector<Route> {
        &self.stored_routes
    }

    pub fn get_routes_mut(&mut self) -> &mut im::Vector<Route> {
        &mut self.stored_routes
    }

    pub fn get_children(&self) -> &im::Vector<ConditionNode> {
        &self.children
    }

    pub fn get_children_mut(&mut self) -> &mut im::Vector<ConditionNode> {
        &mut self.children
    }

    fn _partition_node(
        &mut self,
        mut node: ConditionNode,
        pass: &mut dyn Pass,
        partition_fn: PartitionFn,
    ) -> im::Vector<(ConditionType, ConditionNode)> {
        let mut ret: im::Vector<(ConditionType, ConditionNode)> = im::Vector::new();

        while let Some(child) = node.children.pop_front() {
            let cond = child.condition.clone();
            for partition in self._partition_node(child, pass, partition_fn) {
                if let Some(existing) = ret.iter_mut().find(|x| x.0 == partition.0) {
                    existing.1.children.push_back(partition.1);
                } else {
                    let mut v = im::Vector::new();
                    v.push_back(partition.1);

                    ret.push_back((
                        partition.0.clone(),
                        ConditionNode {
                            children: v,
                            stored_routes: im::Vector::new(),
                            condition: cond.clone(),
                        },
                    ));
                }
            }
        }

        let routes = node.stored_routes.clone();

        for partition in partition_fn(pass, routes) {
            if let Some(existing) = ret.iter_mut().find(|x| x.0 == partition.0) {
                existing.1.stored_routes.append(partition.1);
            } else {
                ret.push_back((
                    partition.0.clone(),
                    ConditionNode {
                        children: im::Vector::new(),
                        stored_routes: partition.1,
                        condition: node.condition.clone(),
                    },
                ));
            }
        }

        ret
    }

    pub fn insert_condition(
        &mut self,
        pass: &mut dyn Pass,
        node: &mut ConditionNode,
        partition_fn: &PartitionFn,
    ) {
        todo!();
    }

    pub fn insert_root_condition(&mut self, pass: &mut dyn Pass, partition_fn: PartitionFn) {
        let mut new_children: im::Vector<ConditionNode> = im::Vector::new();
        while let Some(c) = self.children.pop_front() {
            for partition in self._partition_node(c, pass, partition_fn) {
                let existing_group = new_children.iter_mut().find(|x| x.condition == partition.0);

                match existing_group {
                    Some(existing) => {
                        existing.children.push_back(partition.1);
                    }
                    None => {
                        let mut v = im::Vector::new();
                        v.push_back(partition.1);

                        new_children.push_back(ConditionNode {
                            children: v,
                            stored_routes: im::Vector::new(),
                            condition: partition.0.clone(),
                        });
                    }
                }
            }
        }

        let routes = self.stored_routes.clone();

        for partition in partition_fn(pass, routes) {
            let existing_group = new_children.iter_mut().find(|x| x.condition == partition.0);

            match existing_group {
                Some(existing) => {
                    existing.stored_routes.append(partition.1);
                }
                None => {
                    new_children.push_back(ConditionNode {
                        children: im::Vector::new(),
                        stored_routes: partition.1,
                        condition: partition.0,
                    });
                }
            }
        }

        self.children = new_children;
        self.stored_routes.clear();
    }

    pub fn print_tree(&self) {
        let mut builder = TreeBuilder::new(format!(
            "ConditionTree ({} unpartitioned routes)",
            self.stored_routes.len()
        ));

        for child in &self.children {
            self.build_ptree_nodes(&mut builder, child);
        }

        let tree = builder.build();
        ptree::print_tree(&tree).unwrap();
    }

    fn build_ptree_nodes(&self, builder: &mut TreeBuilder, node: &ConditionNode) {
        builder.begin_child(format!(
            "Condition: {:?} ({} routes)",
            node.condition,
            node.stored_routes.len()
        ));

        for child in &node.children {
            self.build_ptree_nodes(builder, child);
        }

        builder.end_child();
    }

    pub fn run_pass_type<T>(&mut self)
    where
        T: Pass + Default,
    {
        info!("Running {:#?} pass", std::any::type_name::<T>());
        T::run(&mut T::default(), self);
    }
}
