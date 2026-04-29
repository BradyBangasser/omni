use log::debug;
use std::io::Write;
use std::{error::Error, fmt::Binary};

use crate::{
    ctx::OmnicomCtx,
    languages::adapter::{Adapter, OutAdapter, OutWriteContext},
    router::{
        generate::{
            indent_fn,
            tree::{GenNode, GenRoute},
        },
        tree::condition::ConditionType,
    },
};

#[derive(Default)]
pub struct RustAdapter;

impl Adapter for RustAdapter {
    fn get_name(&self) -> &str {
        "RustAdapter"
    }

    fn get_flags(&self) -> u8 {
        0
    }
}

impl RustAdapter {
    pub fn _generate_r<T: std::io::Write>(
        &mut self,
        ctx: &mut OmnicomCtx,
        wctx: &mut OutWriteContext<T>,
        routes: &im::Vector<GenRoute>,
        children: &im::Vector<GenNode>,
    ) -> Result<(), Box<dyn Error>> {
        // Match child routes

        if !routes.is_empty() {
            indent_fn(wctx.w_indent, &mut wctx.writer)?;
            writeln!(wctx.writer, "match route {{").unwrap();
            wctx.w_indent += 1;

            for r in routes {
                indent_fn(wctx.w_indent, &mut wctx.writer)?;
                writeln!(wctx.writer, "b\"{}\" => {{,", r.get_path())?;
                wctx.w_indent += 1;
                self.generate_route(wctx, r)?;
                wctx.w_indent -= 1;
                indent_fn(wctx.w_indent, &mut wctx.writer)?;
                writeln!(wctx.writer, "}}")?;
            }

            indent_fn(wctx.w_indent, &mut wctx.writer)?;
            writeln!(wctx.writer, "_ => {{")?;

            indent_fn(wctx.w_indent, &mut wctx.writer)?;
            writeln!(wctx.writer, "// Handle 404 here")?;

            indent_fn(wctx.w_indent, &mut wctx.writer)?;
            writeln!(wctx.writer, "}},")?;
        }

        if !children.is_empty() {
            if !routes.is_empty() {
                indent_fn(wctx.w_indent, &mut wctx.writer)?;
                writeln!(wctx.writer, "_ => {{")?;
                wctx.w_indent += 1;
            }

            let mut cond_map = im::HashMap::<&'static str, im::Vector<GenNode>>::new();
            for nc in children {
                cond_map
                    .entry(nc.condition.clone().unwrap().get_type())
                    .or_default()
                    .push_front(nc.clone());
            }

            for e in cond_map.values() {
                wctx.w_indent = self.generate_cond(ctx, wctx, e.clone())?;
            }
        }

        if !routes.is_empty() {
            wctx.w_indent -= 1;
            indent_fn(wctx.w_indent, &mut wctx.writer)?;
            writeln!(wctx.writer, "}}").unwrap();
        }

        Ok(())
    }

    fn _generate_length<T: std::io::Write>(
        &mut self,
        ctx: &mut OmnicomCtx,
        wctx: &mut OutWriteContext<T>,
        routes: &im::Vector<GenNode>,
    ) -> Result<usize, Box<dyn Error>> {
        indent_fn(wctx.w_indent, &mut wctx.writer)?;
        writeln!(wctx.writer, "let l = route.len();")?;
        indent_fn(wctx.w_indent, &mut wctx.writer)?;
        writeln!(wctx.writer, "match l {{")?;
        wctx.w_indent += 1;

        for n in routes {
            if let ConditionType::Length(l) = &n.condition.clone().unwrap() {
                indent_fn(wctx.w_indent, &mut wctx.writer)?;
                writeln!(wctx.writer, "{} => {{", l)?;

                wctx.w_indent += 1;

                self._generate_r(ctx, wctx, &n.routes, &n.children)?;

                wctx.w_indent -= 1;

                indent_fn(wctx.w_indent, &mut wctx.writer)?;
                writeln!(wctx.writer, "}},")?;
            }
        }

        indent_fn(wctx.w_indent, &mut wctx.writer)?;
        writeln!(wctx.writer, "_ => {{")?;

        indent_fn(wctx.w_indent, &mut wctx.writer)?;
        writeln!(wctx.writer, "// Handle 404 here")?;

        indent_fn(wctx.w_indent, &mut wctx.writer)?;
        writeln!(wctx.writer, "}},")?;
        wctx.w_indent -= 1;

        indent_fn(wctx.w_indent, &mut wctx.writer)?;
        writeln!(wctx.writer, "}}")?;

        Ok(wctx.w_indent)
    }

    fn format_binary<T>(x: T) -> String
    where
        T: Binary + Sized,
    {
        let size = std::mem::size_of::<T>() * 8;
        let s = format!("{:01$b}", x, size);
        let mut pre = String::from("0b");

        pre.push_str(
            &s.as_bytes()
                .rchunks(4)
                .rev()
                .map(|chunk| std::str::from_utf8(chunk).unwrap_or(""))
                .collect::<Vec<_>>()
                .join("_"),
        );

        pre
    }
}

impl OutAdapter for RustAdapter {
    fn generate(
        &mut self,
        ctx: &mut crate::ctx::OmnicomCtx,
        writer: &mut dyn std::io::Write,
        tree: &crate::router::generate::tree::GenTree,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut wctx = OutWriteContext {
            writer: Vec::<u8>::new(),
            w_indent: 1,
            deps: vec![],
        };

        writeln!(wctx.writer, "pub fn route(method: u32, route: &[u8]) {{")?;
        self._generate_r(ctx, &mut wctx, &tree.node.routes, &tree.node.children)?;
        writeln!(wctx.writer, "}}")?;

        if !wctx.deps.is_empty() {
            for dep in wctx.deps {
                writeln!(writer, "mod {};", dep)?;
            }

            writeln!(writer)?;
        }

        writer.write_all(&wctx.writer)?;

        Ok(())
    }

    fn generate_cond<T: std::io::Write>(
        &mut self,
        ctx: &mut crate::ctx::OmnicomCtx,
        wctx: &mut OutWriteContext<T>,
        routes: im::Vector<GenNode>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        if routes.is_empty() {
            return Ok(wctx.w_indent);
        }

        Ok(match routes[0].condition.clone().unwrap() {
            ConditionType::Length(_) => self._generate_length(ctx, wctx, &routes)?,
            _ => {
                debug!("{:#?}", routes[0]);
                todo!()
            }
        })
    }

    fn generate_route<T: std::io::Write>(
        &mut self,
        wctx: &mut OutWriteContext<T>,
        route: &GenRoute,
    ) -> Result<usize, Box<dyn Error>> {
        let stacks = route.get_stacks();

        if stacks.is_empty() {
            return Err("Unable to generate stack router".into());
        }

        indent_fn(wctx.w_indent, &mut wctx.writer)?;
        let r = stacks.get(0).unwrap();

        wctx.deps.push(r.1.clone());

        writeln!(
            wctx.writer,
            "if method & {} {{",
            Self::format_binary(r.0.bits())
        )?;

        indent_fn(wctx.w_indent + 1, &mut wctx.writer)?;

        writeln!(wctx.writer, "let res = {}.__s_{}();", r.1, r.1)?;

        if stacks.get(1).is_some() {
            for r in stacks.into_iter().skip(1) {
                indent_fn(wctx.w_indent, &mut wctx.writer)?;
                writeln!(
                    wctx.writer,
                    "}} else if method & {} {{",
                    Self::format_binary(r.0.bits())
                )?;
                indent_fn(wctx.w_indent + 1, &mut wctx.writer)?;
                writeln!(wctx.writer, "let res = {}.__s_{}();", r.1, r.1)?;
            }
        }

        indent_fn(wctx.w_indent, &mut wctx.writer)?;

        writeln!(wctx.writer, "}} else {{")?;

        indent_fn(wctx.w_indent + 1, &mut wctx.writer)?;

        writeln!(wctx.writer, "// Handle 405 Here")?;
        indent_fn(wctx.w_indent, &mut wctx.writer)?;

        writeln!(wctx.writer, "}}")?;

        Ok(wctx.w_indent)
    }
}
