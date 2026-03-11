mod ast;
mod ctx;
mod route;
mod treemap;

use std::path::Path;

use crate::{ctx::OmnicomCtx, treemap::walk_routes};

fn main() {
    println!("Starting omnicom.{}", OmnicomCtx::OMNI_VERSION);
    let routes = walk_routes(
        &OmnicomCtx::new(),
        &Path::new("./compiler/test/helloworld/src/routes"),
    )
    .unwrap();
}
