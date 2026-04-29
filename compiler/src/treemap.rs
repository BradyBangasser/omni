use std::{collections::VecDeque, error::Error, path::Path};

use log::{error, info, trace};

use crate::{
    ctx::OmnicomCtx,
    router::{Node, Route, RouteSeg},
};

pub fn walk_routes(ctx: &OmnicomCtx, path: &Path) -> Result<im::Vector<Route>, Box<dyn Error>> {
    info!("Walking {} for routes", path.display());
    let mut rv = im::Vector::new();

    if path.is_dir() {
        rv = _walk_routes_r(ctx, path, im::Vector::new(), rv, im::Vector::new())?;
    }

    Ok(rv)
}

fn _walk_routes_r(
    _ctx: &OmnicomCtx,
    path: &Path,
    parent: im::Vector<RouteSeg>,
    mut rv: im::Vector<Route>,
    mut midq: im::Vector<Node>,
) -> Result<im::Vector<Route>, Box<dyn Error>> {
    let mut dirq = VecDeque::new();
    let mut rq = VecDeque::new();
    for fe in path.read_dir()? {
        let fe = fe?;
        let p = fe.path();

        if p.is_dir() {
            dirq.push_front(p);
        } else {
            let n = Node::from_file(&p);

            if n.is_err() {
                error!("Error parsing potential endpoint {}", p.display());
                continue;
            }

            for n in n? {
                match &n {
                    Node::Middleware(mw) => {
                        trace!("Adding middleware '{}'", mw.file.display());
                        midq.push_back(n);
                    }
                    Node::Endpoint(_, _) => {
                        rq.push_back(n);
                    }
                }
            }
        }
    }

    while let Some(route) = rq.pop_front() {
        rv.push_back(Route::new(parent.clone(), route, midq.clone())?)
    }

    while let Some(dir) = dirq.pop_front() {
        let Some(basename) = dir.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        let entry = if basename.starts_with(':') {
            RouteSeg::Dynamic(basename.to_string())
        } else {
            RouteSeg::Static(basename.to_string())
        };

        let mut pclone = parent.clone();
        pclone.push_back(entry);

        rv.append(_walk_routes_r(
            _ctx,
            &dir,
            pclone,
            im::Vector::new(),
            midq.clone(),
        )?);
    }

    Ok(rv)
}
