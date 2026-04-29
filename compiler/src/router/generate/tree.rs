use std::{collections::HashMap, error::Error};

use base::types::http::Method;

use crate::{
    ctx::OmnicomCtx,
    router::{
        Route,
        generate::stack::StackGenerator,
        tree::condition::{ConditionNode, ConditionTree, ConditionType},
    },
};

#[derive(Clone, Debug)]
pub struct GenRoute {
    path: String,
    // HTTP bitmap with stack id, which will be the same as the path and method
    stack: im::Vector<(Method, String)>,
    // TODO: Add error handling into stacks
}

impl GenRoute {
    pub fn new(path: String) -> Self {
        Self {
            path,
            stack: im::Vector::new(),
        }
    }

    pub fn get_stack_mut(&mut self) -> &mut im::Vector<(Method, String)> {
        &mut self.stack
    }

    pub fn get_stacks(&self) -> &im::Vector<(Method, String)> {
        &self.stack
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

#[derive(Clone, Debug)]
pub struct GenNode {
    pub condition: Option<ConditionType>,
    pub children: im::Vector<GenNode>,
    pub routes: im::Vector<GenRoute>,
}

pub struct GenTree {
    pub node: GenNode,
}

impl GenTree {
    fn _generate_routes(
        ctx: &mut OmnicomCtx,
        sg: &mut StackGenerator,
        routes: &im::Vector<Route>,
    ) -> Result<im::Vector<GenRoute>, Box<dyn Error>> {
        let mut rmap: HashMap<String, GenRoute> = HashMap::new();

        for r in routes {
            let path = r.get_path_str();

            let gr = rmap
                .entry(path.clone())
                .or_insert_with(|| GenRoute::new(path));

            sg.generate_stack(ctx, r, gr)?;
        }

        Ok(rmap.values().cloned().collect())
    }

    fn _from_cond_node_r(
        ctx: &mut OmnicomCtx,
        sg: &mut StackGenerator,
        node: &ConditionNode,
    ) -> Result<GenNode, Box<dyn Error>> {
        let mut children = im::Vector::new();

        for c in &node.children {
            children.push_back(Self::_from_cond_node_r(ctx, sg, c)?);
        }

        Ok(GenNode {
            condition: Some(node.condition.clone()),
            children,
            routes: Self::_generate_routes(ctx, sg, &node.stored_routes)?,
        })
    }

    pub fn from_cond_tree(
        ctx: &mut OmnicomCtx,
        sg: &mut StackGenerator,
        tree: &mut ConditionTree,
    ) -> Result<GenTree, Box<dyn Error>> {
        let mut gn = GenNode {
            condition: None,
            children: im::Vector::new(),
            routes: Self::_generate_routes(ctx, sg, tree.get_routes())?,
        };

        for n in tree.get_children() {
            gn.children.push_back(Self::_from_cond_node_r(ctx, sg, n)?);
        }

        sg.generate_build(ctx)?;
        Ok(GenTree { node: gn })
    }
}
