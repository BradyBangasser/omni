mod ast;
mod build;
mod ctx;
mod languages;
mod router;
mod treemap;

use std::fs::OpenOptions;
use std::io::{Write, stdout};
use std::path::Path;

use crate::languages::adapter::{OutAdapter, OutWriteContext};
use crate::languages::go::GoAdapter;
use crate::languages::rust::RustAdapter;
use crate::router::generate::stack::StackGenerator;
use crate::router::generate::tree::GenTree;
use crate::router::pass;
use crate::router::tree::condition::ConditionTree;
use crate::{ctx::OmnicomCtx, treemap::walk_routes};

fn main() {
    let mut ctx = OmnicomCtx::default();
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();

    println!("Starting omnicom.{}", OmnicomCtx::OMNI_VERSION);
    let routes = walk_routes(&ctx, &Path::new("./compiler/test/helloworld/src/routes")).unwrap();

    let mut tree = ConditionTree::new(routes);
    // tree.run_pass_type::<pass::segcount::Segcount>();
    // tree.run_pass_type::<pass::method::Method>();
    tree.run_pass_type::<pass::length::Length>();

    let mut sg = StackGenerator::default();

    sg.register_default_adapter::<GoAdapter>();

    let gentree = GenTree::from_cond_tree(&mut ctx, &mut sg, &mut tree).unwrap();
    let mut ra = RustAdapter::default();

    let mut d = ctx.get_lib().to_path_buf();
    d.push("mod.rs");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&d)
        .unwrap();

    ra.generate(&mut ctx, &mut file, &gentree).unwrap();
}
