use crate::router::{
    Route,
    generate::{Generator, GeneratorFormat, indent_fn},
    tree::condition::ConditionNode,
};

#[derive(Default, Debug)]
pub struct Format;

impl Format {
    pub fn format_r<T: std::io::Write>(
        writer: &mut T,
        mut indent: usize,
        routes: &im::Vector<Route>,
        children: &im::Vector<ConditionNode>,
    ) {
        // Match child routes
        let routes_empty = routes.is_empty();
        if !routes_empty {
            indent_fn(indent, writer).unwrap();
            writeln!(writer, "match route {{").unwrap();
            indent += 1;

            for r in routes {
                indent_fn(indent, writer).unwrap();
                writeln!(writer, "b\"{}\" => {{}},", r.get_path_str()).unwrap();
            }

            indent_fn(indent, writer).unwrap();
            writeln!(writer, "_ => {{}},").unwrap();
        }

        if !children.is_empty() {
            if !routes_empty {
                indent_fn(indent, writer).unwrap();
                writeln!(writer, "_ => {{").unwrap();
                indent += 1;
            }

            let mut cond_map = im::HashMap::<&'static str, im::Vector<ConditionNode>>::new();
            for nc in children {
                cond_map
                    .entry(nc.condition.get_type())
                    .or_default()
                    .push_front(nc.clone());
            }

            for e in cond_map.values() {
                indent = e[0].condition.gen_code(e, indent, writer).unwrap();
            }
        }

        if !routes_empty {
            indent -= 1;
            indent_fn(indent, writer).unwrap();
            writeln!(writer, "}}").unwrap();
        }
    }
}

impl GeneratorFormat for Format {
    fn format<T: std::io::Write>(&mut self, g: &mut Generator<T>) {
        writeln!(
            g.writer,
            "#[inline(always)]\npub fn route(method: u8, route: &[u8]) {{"
        )
        .unwrap();
        g.indent += 1;

        Self::format_r(
            &mut g.writer,
            g.indent,
            g.tree.get_routes(),
            g.tree.get_children(),
        );

        g.indent -= 1;
        indent_fn(g.indent, &mut g.writer).unwrap();
        writeln!(g.writer, "}}").unwrap();
    }
}
