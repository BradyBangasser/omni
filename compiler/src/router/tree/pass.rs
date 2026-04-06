use std::fmt::Debug;

use crate::router::tree::condition::ConditionTree;

pub trait Pass: Debug {
    fn run(&mut self, tree: &mut ConditionTree);
}
