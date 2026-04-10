use std::error::Error;

use bimap::BiHashMap;
use im::Vector;

use crate::{
    build::OmniBuilder,
    ctx::OmnicomCtx,
    languages::adapter::Adapter,
    router::{Node, Route, generate::tree::GenNode},
};

pub struct StackGenerator {
    builder: OmniBuilder,
    adapters: Vec<Box<dyn Adapter>>,
    pregen_plugins: Vec<String>,
    postgen_plugins: Vec<String>,
}

impl StackGenerator {
    pub fn generate_stack(
        &mut self,
        ctx: &mut OmnicomCtx,
        r: &Route,
    ) -> Result<GenNode, Box<dyn Error>> {
        // Keep track of variables in the stack
        let mut vars: BiHashMap<String, String> = BiHashMap::new();

        vars.insert("HotContext".into(), "hc".into());
        vars.insert("ColdContext".into(), "cc".into());

        let mut gencode: Vec<u8> = vec![];

        let mut indent = 1;

        for c in &r.chain {
            let adapter = self
                .adapters
                .iter_mut()
                .find(|a| a.handles(c.get_src_path()))
                .ok_or_else(|| {
                    format!(
                        "Missing LanguageAdaptor for source path: {}",
                        c.get_src_path().display()
                    )
                })?;

            adapter.configure_build(ctx, &mut self.builder);

            indent = adapter.emit(ctx, &mut gencode, indent, &mut vars, c)?;
        }

        todo!();
    }
}
