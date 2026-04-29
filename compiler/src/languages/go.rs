use crate::{
    build::OmniBuilder,
    ctx::OmnicomCtx,
    languages::adapter::{Adapter, AdapterEmit, AdapterStackCtx, InAdapter},
    router::{Node, generate::indent_fn},
};

use const_hex::encode;
use log::debug;
use sha3::digest::Update as Sha3Update;
use sha3::{
    Digest, Sha3_256, Shake128,
    digest::{ExtendableOutput, XofReader},
};
use std::path::Path;
use std::process::Command;
use std::{error::Error, fs::OpenOptions};
use std::{io::Write, process::Stdio};

#[derive(Default)]
pub struct GoAdapter;

impl GoAdapter {
    const MODULE_NAME: &'static str = match option_env!("GO_MODULE_NAME") {
        Some(name) => name,
        None => "omni",
    };

    fn _generate_mod(&self, p: &Path) -> Result<(), Box<dyn Error>> {
        debug!(
            "Generating go.mod in {} (package '{}')",
            p.display(),
            GoAdapter::MODULE_NAME
        );
        let mut mod_file = p.to_path_buf();
        mod_file.push("go.mod");

        if mod_file.exists() {
            std::fs::remove_file(mod_file)?;
        }

        Command::new("go")
            .arg("mod")
            .arg("init")
            .arg(GoAdapter::MODULE_NAME)
            .current_dir(p)
            .env("CGO_ENABLED", "1")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        Command::new("go")
            .arg("mod")
            .arg("tidy")
            .current_dir(p)
            .env("CGO_ENABLED", "1")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        Ok(())
    }
}

impl InAdapter for GoAdapter {
    // Generates rust stack
    fn emit(
        &mut self,
        _ctx: &mut crate::ctx::OmnicomCtx,
        writer: &mut dyn std::io::Write,
        actx: &mut AdapterStackCtx,
        mut indent: usize,
        n: &crate::router::Node,
    ) -> Result<(usize, AdapterEmit), Box<dyn std::error::Error>> {
        let nd = match n {
            Node::Endpoint(nd, _) => nd,
            Node::Middleware(nd) => nd,
        };

        let package_name = "routes";

        let mut hasher = Sha3_256::default();
        sha3::digest::Update::update(&mut hasher, n.get_src_path().to_str().unwrap().as_bytes());

        let mut phasher = Sha3_256::default();
        sha3::digest::Update::update(
            &mut phasher,
            format!("{}-{}", nd.file.display(), nd.fname).as_bytes(),
        );

        let pid = encode(phasher.finalize());

        let mut write_row = |indent: usize, text: &str| -> Result<(), Box<dyn Error>> {
            indent_fn(indent, writer)?;
            sha3::digest::Update::update(&mut hasher, text.as_bytes());
            writeln!(writer, "{}", text)?;
            Ok(())
        };

        write_row(indent, "unsafe {")?;
        indent += 1;

        let handle = format!("{}_{}_{}", package_name, nd.fname, pid);

        write_row(indent, &format!("let res = {}();", handle))?;

        indent -= 1;

        write_row(indent, "}")?;

        let mut logic_hasher = Shake128::default();
        let content = std::fs::read(n.get_src_path())?;

        logic_hasher.update(&content);

        let mut reader = logic_hasher.finalize_xof();

        let mut file_hash = [0u8; 8];

        reader.read(&mut file_hash);

        let file_name = format!("{}.go", encode(file_hash));

        actx.file(&file_name).write_all(&content)?;

        let mut adapter_emit = AdapterEmit::new(
            &nd.file,
            nd.fname.clone(),
            "1".into(),
            hasher.finalize().to_vec(),
        );

        adapter_emit.set_ctx("handle", &handle);

        actx.ffi.push(handle);

        Ok((indent, adapter_emit))
    }

    fn handles(&self, p: &std::path::Path) -> bool {
        p.extension().is_some() && p.extension().unwrap() == "go"
    }

    fn configure_build(
        &mut self,
        ctx: &mut OmnicomCtx,
        builder: &mut OmniBuilder,
        stacks: &std::collections::HashMap<String, Vec<AdapterEmit>>,
    ) -> Result<(), Box<dyn Error>> {
        if stacks.is_empty() {
            return Err("Stack list is empty".into());
        }

        let mut d = ctx.get_lib().to_path_buf();

        d.push("main.go");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&d)?;

        let mut exports = vec![];

        writeln!(file, "package main")?;

        write!(file, "import ")?;

        writeln!(file, "(")?;
        writeln!(file, "    \"C\"")?;
        writeln!(file, "    \"fmt\"")?;

        let mut write_row = |indent: usize, text: &str| -> Result<(), Box<dyn Error>> {
            indent_fn(indent, &mut exports)?;
            writeln!(exports, "{}", text)?;
            Ok(())
        };

        for stack in stacks {
            writeln!(
                file,
                "    __{} \"{}/{}\"",
                stack.0,
                Self::MODULE_NAME,
                stack.0
            )?;

            for ae in stack.1 {
                let handle = ae.get_ctx("handle").unwrap();

                write_row(0, &format!("//export {}", handle))?;
                // TODO: Add param parsing
                write_row(0, &format!("func {}() {{", handle))?;

                write_row(
                    1,
                    &format!(
                        "__{}.{}(__{}.OmniContext{{}});",
                        stack.0,
                        ae.get_func_name(),
                        stack.0,
                    ),
                )?;

                write_row(0, "}\n\n")?;
            }
        }

        writeln!(file, ")\n")?;

        file.write_all(&exports)?;

        writeln!(file)?;

        writeln!(file, "func main() {{")?;
        writeln!(file, "    fmt.Println(\"dont run me im sensitive\")")?;
        writeln!(file, "}}")?;

        self._generate_mod(ctx.get_lib())?;

        Ok(())
    }
}

impl Adapter for GoAdapter {
    fn get_name(&self) -> &str {
        "GoAdapter"
    }

    fn get_flags(&self) -> u8 {
        0
    }
}
