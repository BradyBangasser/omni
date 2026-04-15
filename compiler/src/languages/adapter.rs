use std::{error::Error, path::Path};

use bimap::BiHashMap;
use const_hex::encode;

use crate::{
    build::OmniBuilder,
    ctx::OmnicomCtx,
    router::{
        Node,
        generate::tree::{GenNode, GenRoute, GenTree},
    },
};

pub struct AdapterEmit {
    version: String,
    hash: Vec<u8>,
}

impl AdapterEmit {
    pub fn new(version: String, hash: Vec<u8>) -> Self {
        Self { version, hash }
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_hash(&self) -> &Vec<u8> {
        &self.hash
    }

    pub fn get_hash_str(&self) -> String {
        encode(&self.hash)
    }
}

#[derive(Default)]
pub struct AdapterStackCtx {
    pub vars: BiHashMap<String, String>,
    pub ffi: Vec<String>,
}

pub enum AdapterFlags {
    Importable = 0x0,
    RequiresBuild = 0x1,
    Object = 0x2,
    Interpretted = 0x4,
}

pub trait Adapter {
    fn get_version(&self) -> &str {
        "0.0.1"
    }

    fn get_name(&self) -> &str;

    fn get_flags(&self) -> u8;

    fn configure_build(&mut self, ctx: &mut OmnicomCtx, builder: &mut OmniBuilder);
}

pub trait InAdapter: Send + Sync + Adapter {
    fn emit(
        &mut self,
        ctx: &mut OmnicomCtx,
        writer: &mut dyn std::io::Write,
        actx: &mut AdapterStackCtx,
        indent: usize,
        n: &Node,
    ) -> Result<(usize, AdapterEmit), Box<dyn Error>>;
    fn handles(&self, p: &Path) -> bool;
}

pub trait OutAdapter: Send + Sync + Adapter {
    fn generate(
        &mut self,
        ctx: &mut OmnicomCtx,
        writer: &mut dyn std::io::Write,
        tree: &GenTree,
    ) -> Result<(), Box<dyn Error>>;

    fn generate_cond(
        &mut self,
        ctx: &mut OmnicomCtx,
        indent: usize,
        writer: &mut dyn std::io::Write,
        routes: im::Vector<GenNode>,
    ) -> Result<usize, Box<dyn Error>>;
}
