use std::path::PathBuf;

use crate::router::tree::condition::ConditionNode;

pub struct GenRoute {
    path: String,

    // Localized path based off parent
    lpath: String,

    // Generated Code location
    gencode: Option<PathBuf>,

    // HTTP bitmap with handler name
    stack: im::Vector<(u8, String)>,

    // HTTP Accept bitmap
    accept: u8,

    // Error Handling Stack
    estack: im::Vector<(u8, String)>,

    // Type (0 == handler, 1 == error) to source
    src: im::HashMap<(u8, String), PathBuf>,
}

pub struct GenNode {
    condition: ConditionNode,
    children: im::Vector<GenNode>,
    routes: im::Vector<GenRoute>,
}

pub struct GenTree {}
