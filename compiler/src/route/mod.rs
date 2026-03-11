use std::{
    collections::LinkedList,
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use core::types::http::Method;

use tree_sitter::Tree;

use crate::ast::{self, get_func_params};

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum RouteSeg {
    Dynamic(String),
    Static(String),
}

impl fmt::Display for RouteSeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteSeg::Static(s) => write!(f, "{s}"),
            RouteSeg::Dynamic(s) => write!(f, ":{s}"),
        }
    }
}

pub enum Flags {
    DB = 0x01,
    Ctx = 0x02,
    Hdr = 0x04,
    Bdy = 0x08,
    Dyn = 0x10,
}

#[derive(Debug, Clone)]
pub enum Node {
    Endpoint(NodeDatum),
    Middleware(NodeDatum),
}

impl Node {
    pub fn from_file(p: &Path) -> Result<im::Vector<Node>, Box<dyn Error>> {
        let file_method = Method::parse(
            p.file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or_default(),
        );

        let mut nv = im::Vector::new();

        let src = fs::read_to_string(p)?;

        let tree = ast::parse(&src)?;

        let dfuncs = ast::discover_functions(&tree, &src)?;

        println!("{}", p.display());
        for f in dfuncs {
            println!("{}", f.name);
            for p in get_func_params(f.declaration, &src) {
                println!("{}", p);
            }
        }

        let mut nd = NodeDatum {
            file: PathBuf::from(p),
            fname: String::from(""),
            params: LinkedList::new(),
            ret: LinkedList::new(),
            ruflags: 0,
            ast: tree,
        };

        Ok(nv)
    }
}

#[derive(Debug, Clone)]
pub struct NodeDatum {
    pub file: PathBuf,
    pub fname: String,
    pub params: LinkedList<(String, String)>,
    pub ret: LinkedList<String>,
    pub ruflags: u64, // Usage flags from the current module flag struct
    pub ast: Tree,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub chain: im::Vector<Node>,
    pub method: Method,
    pub path: im::Vector<RouteSeg>,
}

impl Route {
    pub fn new(path: im::Vector<RouteSeg>, ep: Node, chain: im::Vector<Node>) -> Self {
        let mut r = Route {
            chain,
            path,
            method: Method::GET,
        };

        r.chain.push_back(ep);

        r
    }
}
