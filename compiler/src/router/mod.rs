pub mod generate;
pub mod pass;
pub mod tree;

use std::{
    collections::LinkedList,
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use base::types::http::Method;

use log::{info, trace};
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
    Hot = 0x20,
}

#[derive(Debug, Clone)]
pub enum Node {
    Endpoint(NodeDatum, Method),
    Middleware(NodeDatum),
}

impl Node {
    pub fn from_file(p: &Path) -> Result<im::Vector<Node>, Box<dyn Error>> {
        info!("Parsing routes from {}", p.display());
        let file_method = Method::parse(
            p.file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or_default(),
        );

        let mut nv = im::Vector::new();

        let src = fs::read_to_string(p)?;

        let tree = ast::parse(&src)?;

        let dfuncs = ast::discover_functions(&tree, &src)?;

        for f in dfuncs {
            trace!("Discovered possible endpoint, {}:{}", p.display(), f.name);
            let params = get_func_params(f.declaration, &src);

            if params[0] != "OmniContext" {
                println!(
                    "{} is not a valid handler (Doesn't take OmniContext)",
                    f.name
                );
                continue;
            }

            let mut nd = NodeDatum {
                file: PathBuf::from(p),
                fname: String::from(""),
                params: LinkedList::new(),
                ret: LinkedList::new(),
                ruflags: 0,
                ast: tree.clone(), // Maybe use reference counted datatype
            };

            nd.fname = f.name.clone();
            if let Some(m) = Method::parse(&f.name) {
                // it's an endpoint
                trace!(
                    "Viable {:?} (function name) endpoint: {}:{}",
                    m,
                    p.display(),
                    f.name
                );
                nv.push_back(Self::Endpoint(nd, m));
            } else if let Some(m) = file_method.clone() {
                // it's an endpoint, do the same as before
                trace!(
                    "Viable {:?} (file name) endpoint: {}:{}",
                    m,
                    p.display(),
                    f.name
                );
                nv.push_back(Self::Endpoint(nd, m));
            } else {
                // It's a middleware function
                trace!("Viable middleware: {}:{}", p.display(), f.name);
                nv.push_back(Self::Middleware(nd));
            }
        }

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
    pub fn new(
        path: im::Vector<RouteSeg>,
        ep: Node,
        chain: im::Vector<Node>,
    ) -> Result<Self, Box<dyn Error>> {
        if let Node::Endpoint(_, m) = &ep {
            let mut r = Route {
                chain,
                path,
                method: m.clone(),
            };

            r.chain.push_back(ep);

            Ok(r)
        } else {
            Err("".into())
        }
    }

    pub fn get_path_str(&self) -> String {
        let mut r: String = self
            .path
            .iter()
            .map(|x| match x {
                RouteSeg::Dynamic(s) => String::from("/") + &s.clone(),
                RouteSeg::Static(s) => String::from("/") + &s.clone(),
            })
            .collect();

        if r.is_empty() {
            r.push('/');
        }

        r
    }
}
