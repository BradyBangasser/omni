pub mod generate;
pub mod pass;
pub mod tree;

use bitflags::bitflags;
use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use base::types::http::Method;
use log::{info, trace, warn};
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

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RouteFlags: u64 {
        const DB  = 0x01;
        const Ctx = 0x02;
        const Hdr = 0x04;
        const Bdy = 0x08;
        const Dyn = 0x10;
        const Hot = 0x20;
    }
}

#[derive(Debug, Clone)]
pub struct NodeDatum {
    pub file: PathBuf,
    pub fname: String,
    pub params: Vec<(String, String)>,
    pub ret: Vec<String>,
    pub ruflags: RouteFlags,
    pub ast: Tree,
}

#[derive(Debug, Clone)]
pub enum Node {
    Endpoint(NodeDatum, Method),
    Middleware(NodeDatum),
}

// --- NEW GETTERS ADDED HERE ---
impl Node {
    pub fn get_src_path(&self) -> &Path {
        match self {
            Node::Endpoint(d, _) | Node::Middleware(d) => &d.file,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Node::Endpoint(d, _) | Node::Middleware(d) => &d.fname,
        }
    }

    pub fn get_params(&self) -> &[(String, String)] {
        match self {
            Node::Endpoint(d, _) | Node::Middleware(d) => &d.params,
        }
    }

    pub fn get_returns(&self) -> &[String] {
        match self {
            Node::Endpoint(d, _) | Node::Middleware(d) => &d.ret,
        }
    }

    pub fn get_flags(&self) -> RouteFlags {
        match self {
            Node::Endpoint(d, _) | Node::Middleware(d) => d.ruflags,
        }
    }

    pub fn is_middleware(&self) -> bool {
        matches!(self, Node::Middleware(_))
    }

    pub fn get_method(&self) -> Option<&Method> {
        match self {
            Node::Endpoint(_, m) => Some(m),
            Node::Middleware(_) => None,
        }
    }
    // ------------------------------

    pub fn from_file(p: &Path) -> Result<im::Vector<Node>, Box<dyn Error>> {
        info!("Parsing routes from {}", p.display());
        let file_method = Method::from_str(
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

            if params.is_empty() || params[0] != "OmniContext" {
                warn!(
                    "{} is not a valid handler (Doesn't take OmniContext)",
                    f.name
                );
                continue;
            }

            let mut nd = NodeDatum {
                file: PathBuf::from(p),
                fname: f.name.clone(),
                params: Vec::new(),
                ret: Vec::new(),
                ruflags: RouteFlags::empty(),
                ast: tree.clone(),
            };

            if let Ok(m) = Method::from_str(&f.name) {
                trace!(
                    "Viable {:?} (function name) endpoint: {}:{}",
                    m,
                    p.display(),
                    f.name
                );
                nv.push_back(Self::Endpoint(nd, m));
            } else if let Ok(m) = file_method.clone() {
                trace!(
                    "Viable {:?} (file name) endpoint: {}:{}",
                    m,
                    p.display(),
                    f.name
                );
                nv.push_back(Self::Endpoint(nd, m));
            } else {
                trace!("Viable middleware: {}:{}", p.display(), f.name);
                nv.push_back(Self::Middleware(nd));
            }
        }

        Ok(nv)
    }

    pub fn to_str(&self) -> String {
        match self {
            Node::Endpoint(nd, m) => format!("ENDPOINT   [{:<6}] -> {}", m, nd.file.display()),
            Node::Middleware(nd) => format!("MIDDLEWARE [ANY   ] -> {}", nd.file.display()),
        }
    }
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
            Err("Provided Node is not an Endpoint".into())
        }
    }

    pub fn get_path_str(&self) -> String {
        let mut r: String = self.path.iter().map(|seg| format!("/{}", seg)).collect();

        if r.is_empty() {
            r.push('/');
        }

        r
    }
}
