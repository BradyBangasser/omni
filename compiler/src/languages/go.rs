use std::error::Error;
use ulid::Ulid;

use sha3::{Digest, Sha3_256};

use crate::{
    languages::adapter::{Adapter, AdapterEmit, AdapterStackCtx, InAdapter},
    router::{Node, generate::indent_fn},
};

#[derive(Default)]
pub struct GoAdapter {}

impl InAdapter for GoAdapter {
    // Generates rust stack
    fn emit(
        &mut self,
        _ctx: &mut crate::ctx::OmnicomCtx,
        writer: &mut dyn std::io::Write,
        actx: &mut AdapterStackCtx,
        mut indent: usize,
        n: &crate::router::Node,
    ) -> Result<(usize, AdapterEmit), Box<dyn std::error::Error>> {
        let nd = match n {
            Node::Endpoint(nd, _) => nd,
            Node::Middleware(nd) => nd,
        };

        let mut hasher = Sha3_256::default();
        hasher.update(n.get_src_path().to_str().unwrap().as_bytes());

        let mut write_row = |indent: usize, text: &str| -> Result<(), Box<dyn Error>> {
            indent_fn(indent, writer)?;
            hasher.update(text.as_bytes());
            writeln!(writer, "{}", text)?;
            Ok(())
        };

        write_row(indent, "unsafe {")?;
        indent += 1;

        let handle = format!("{}_{}_{}", "PACKAGE_NAME_HERE", nd.fname, Ulid::new());

        write_row(indent, &format!("let res = {}();", handle))?;
        actx.ffi.push(handle);

        indent -= 1;

        write_row(indent, "}")?;

        Ok((
            indent,
            AdapterEmit::new("1".into(), hasher.finalize().to_vec()),
        ))
    }

    fn handles(&self, p: &std::path::Path) -> bool {
        true
    }
}

impl Adapter for GoAdapter {
    fn get_name(&self) -> &str {
        "GoAdapter"
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
