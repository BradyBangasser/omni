mod ast;
mod ctx;
mod router;
mod treemap;

use std::io::stdout;
use std::path::Path;

use crate::router::generate::Generator;
use crate::router::tree::condition::ConditionTree;
use crate::router::{generate, pass};
use crate::{ctx::OmnicomCtx, treemap::walk_routes};
use log::debug;

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();

    println!("Starting omnicom.{}", OmnicomCtx::OMNI_VERSION);
    let routes = walk_routes(
        &OmnicomCtx::new(),
        &Path::new("./compiler/test/helloworld/src/routes"),
    )
    .unwrap();

    let mut tree = ConditionTree::new(routes);
    // tree.run_pass_type::<pass::segcount::Segcount>();
    tree.run_pass_type::<pass::method::Method>();
    tree.run_pass_type::<pass::length::Length>();

    Generator::new(tree, stdout()).default_to::<generate::format::rs::Format>();
}
