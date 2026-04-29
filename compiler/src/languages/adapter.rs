use std::{
    error::Error,
    path::{Path, PathBuf},
};

use bimap::BiHashMap;
use const_hex::encode;
use im::HashMap;

use crate::{
    build::OmniBuilder,
    ctx::OmnicomCtx,
    router::{
        Node,
        generate::tree::{GenNode, GenRoute, GenTree},
    },
};

#[derive(Clone, Default)]
pub struct AdapterEmit {
    version: String,
    hash: Vec<u8>,
    _src_path: PathBuf,
    func_name: String,
    ctx: im::HashMap<String, String>,
}

pub struct OutWriteContext<T: std::io::Write> {
    pub writer: T,
    pub w_indent: usize,

    pub deps: Vec<String>,
}

impl AdapterEmit {
    pub fn new(src_path: &Path, func_name: String, version: String, hash: Vec<u8>) -> Self {
        Self {
            version,
            hash,
            _src_path: src_path.to_path_buf(),
            func_name,
            ctx: im::HashMap::new(),
        }
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

    pub fn get_func_name(&self) -> &String {
        &self.func_name
    }

    pub fn get_ctx(&self, key: &str) -> Option<&String> {
        self.ctx.get(key)
    }

    pub fn set_ctx(&mut self, key: &str, val: &str) {
        self.ctx.insert(key.into(), val.into());
    }
}

#[derive(Default)]
pub struct AdapterStackCtx {
    pub vars: BiHashMap<String, String>,
    pub ffi: Vec<String>,
    files: HashMap<String, Vec<u8>>,
}

impl AdapterStackCtx {
    pub fn file(&mut self, f: &str) -> &mut Vec<u8> {
        self.files.entry(f.to_owned()).or_default()
    }

    pub fn get_files(&self) -> &HashMap<String, Vec<u8>> {
        &self.files
    }
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

    fn configure_build(
        &mut self,
        ctx: &mut OmnicomCtx,
        builder: &mut OmniBuilder,
        stacks: &std::collections::HashMap<String, Vec<AdapterEmit>>,
    ) -> Result<(), Box<dyn Error>>;
}

pub trait OutAdapter: Send + Sync + Adapter {
    fn generate(
        &mut self,
        ctx: &mut OmnicomCtx,
        writer: &mut dyn std::io::Write,
        tree: &GenTree,
    ) -> Result<(), Box<dyn Error>>;

    fn generate_cond<T: std::io::Write>(
        &mut self,
        ctx: &mut OmnicomCtx,
        wctx: &mut OutWriteContext<T>,
        routes: im::Vector<GenNode>,
    ) -> Result<usize, Box<dyn Error>>;

    fn generate_route<T: std::io::Write>(
        &mut self,
        wctx: &mut OutWriteContext<T>,
        route: &GenRoute,
    ) -> Result<usize, Box<dyn Error>>;
}
