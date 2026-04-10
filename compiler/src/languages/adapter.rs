use std::{error::Error, path::Path};

use bimap::BiHashMap;

use crate::{build::OmniBuilder, ctx::OmnicomCtx, router::Node};

#[repr(u8)]
pub enum AdapterFlags {
    Importable = 0x0,
    RequiresBuild = 0x1,
    Object = 0x2,
    Interpretted = 0x4,
}

pub trait Adapter: Send + Sync {
    fn get_flags(&self) -> u8;

    fn handles(&self, p: &Path) -> bool;

    fn configure_build(&mut self, ctx: &mut OmnicomCtx, builder: &mut OmniBuilder);

    fn emit(
        &mut self,
        ctx: &mut OmnicomCtx,
        writer: &mut dyn std::io::Write,
        indent: usize,
        vars: &mut BiHashMap<String, String>,
        n: &Node,
    ) -> Result<usize, Box<dyn Error>>;
}
