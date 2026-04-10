use bimap::BiHashMap;

use crate::{
    languages::adapter::Adapter,
    router::{
        Node,
        generate::{Generator, indent_fn},
    },
};

pub struct GoAdapter {}

impl Adapter for GoAdapter {
    fn get_flags(&self) -> u8 {
        0
    }

    fn handles(&self, p: &std::path::Path) -> bool {
        true
    }

    fn configure_build(
        &mut self,
        ctx: &mut crate::ctx::OmnicomCtx,
        builder: &mut crate::build::OmniBuilder,
    ) {
    }

    fn emit(
        &mut self,
        ctx: &mut crate::ctx::OmnicomCtx,
        writer: &mut dyn std::io::Write,
        mut indent: usize,
        vars: &mut BiHashMap<String, String>,
        n: &crate::router::Node,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let nd = match n {
            Node::Endpoint(nd, _) => nd,
            Node::Middleware(nd) => nd,
        };

        indent_fn(indent, writer)?;
        writeln!(writer, "unsafe {{")?;
        indent += 1;

        indent_fn(indent, writer)?;
        writeln!(writer, "let res = {}();", nd.fname)?;

        indent -= 1;
        indent_fn(indent, writer)?;
        writeln!(writer, "}}")?;

        Ok(indent)
    }
}
