use std::error::Error;

use crate::{
    ctx::OmnicomCtx,
    languages::adapter::{Adapter, OutAdapter},
    router::{
        generate::{
            indent_fn,
            tree::{GenNode, GenRoute},
        },
        tree::condition::ConditionType,
    },
};

#[derive(Default)]
pub struct RustAdapter;

impl Adapter for RustAdapter {
    fn get_name(&self) -> &str {
        "RustAdapter"
    }

    fn get_flags(&self) -> u8 {
        0
    }

    fn configure_build(
        &mut self,
        _ctx: &mut crate::ctx::OmnicomCtx,
        _builder: &mut crate::build::OmniBuilder,
    ) {
    }
}

impl RustAdapter {
    pub fn _generate_r(
        &mut self,
        ctx: &mut OmnicomCtx,
        writer: &mut dyn std::io::Write,
        mut indent: usize,
        routes: &im::Vector<GenRoute>,
        children: &im::Vector<GenNode>,
    ) -> Result<(), Box<dyn Error>> {
        // Match child routes
        let routes_empty = routes.is_empty();
        if !routes_empty {
            indent_fn(indent, writer).unwrap();
            writeln!(writer, "match route {{").unwrap();
            indent += 1;

            for r in routes {
                indent_fn(indent, writer).unwrap();
                writeln!(writer, "b\"{}\" => {{}},", r.get_path()).unwrap();
            }

            indent_fn(indent, writer).unwrap();
            writeln!(writer, "_ => {{}},").unwrap();
        }

        if !children.is_empty() {
            if !routes_empty {
                indent_fn(indent, writer).unwrap();
                writeln!(writer, "_ => {{").unwrap();
                indent += 1;
            }

            let mut cond_map = im::HashMap::<&'static str, im::Vector<GenNode>>::new();
            for nc in children {
                cond_map
                    .entry(nc.condition.clone().unwrap().get_type())
                    .or_default()
                    .push_front(nc.clone());
            }

            for e in cond_map.values() {
                indent = self.generate_cond(ctx, indent, writer, e.clone())?;
            }
        }

        if !routes_empty {
            indent -= 1;
            indent_fn(indent, writer).unwrap();
            writeln!(writer, "}}").unwrap();
        }

        Ok(())
    }

    fn _generate_segcount(
        &self,
        indent: usize,
        writer: &mut dyn std::io::Write,
        routes: &im::Vector<GenNode>,
    ) -> Result<usize, Box<dyn Error>> {
        todo!()
    }
}

impl OutAdapter for RustAdapter {
    fn generate(
        &mut self,
        ctx: &mut crate::ctx::OmnicomCtx,
        writer: &mut dyn std::io::Write,
        tree: &crate::router::generate::tree::GenTree,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self._generate_r(ctx, writer, 1, &tree.node.routes, &tree.node.children)?;
        Ok(())
    }

    fn generate_cond(
        &mut self,
        _ctx: &mut crate::ctx::OmnicomCtx,
        indent: usize,
        writer: &mut dyn std::io::Write,
        routes: im::Vector<GenNode>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        if routes.is_empty() {
            return Ok(indent);
        }

        Ok(match routes[0].condition.clone().unwrap() {
            ConditionType::SegmentCount(_) => self._generate_segcount(indent, writer, &routes)?,
            _ => todo!(),
        })
    }
}
