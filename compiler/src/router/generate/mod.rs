pub mod format;
pub mod stack;
pub mod tree;

use std::fmt::Debug;

use log::trace;

use crate::router::tree::condition::ConditionTree;

pub fn indent_fn<W: ?Sized + std::io::Write>(indent: usize, writer: &mut W) -> std::io::Result<()> {
    write!(writer, "{:width$}", "", width = indent * 4)
}

pub struct Generator<T> {
    pub indent: usize,
    pub tree: ConditionTree,
    pub writer: T,
}

pub trait GeneratorFormat: Debug {
    fn format<T: std::io::Write>(&mut self, g: &mut Generator<T>);
}

impl<T: std::io::Write> Generator<T> {
    pub fn new(tree: ConditionTree, writer: T) -> Generator<T> {
        trace!("Creating new generator");
        Generator {
            indent: 0,
            tree,
            writer,
        }
    }

    pub fn default_to<F>(&mut self)
    where
        F: GeneratorFormat + Default,
    {
        trace!(
            "Outputting the tree to {:#?} (default contructor)",
            std::any::type_name::<F>()
        );
        self.to(&mut F::default());
    }

    pub fn to<F>(&mut self, formatter: &mut F)
    where
        F: GeneratorFormat,
    {
        trace!("Outputting the tree to {:#?}", std::any::type_name::<F>());
        formatter.format(self);
    }
}
