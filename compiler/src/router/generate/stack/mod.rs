use bimap::BiHashMap;
use chrono::Utc;
use const_hex::encode;
use log::{trace, warn};
use sha3::{Digest, Sha3_256};
use std::fmt::Write as FmtWrite;
use std::fs::OpenOptions;
use std::io::Write as IoWrite;
use std::{error::Error, fs};

use crate::languages::adapter::AdapterStackCtx;
use crate::router::generate::indent_fn;
use crate::router::generate::tree::GenRoute;
use crate::{build::OmniBuilder, ctx::OmnicomCtx, languages::adapter::InAdapter, router::Route};

pub struct StackGenerator {
    builder: OmniBuilder,
    adapters: Vec<Box<dyn InAdapter>>,
    pregen_plugins: Vec<String>,
    postgen_plugins: Vec<String>,
}

impl StackGenerator {
    pub fn register_default_adapter<T: InAdapter + Default + 'static>(&mut self) {
        self.register_adapter(Box::new(T::default()));
    }

    pub fn register_adapter(&mut self, adapter: Box<dyn InAdapter>) {
        self.adapters.push(adapter);
    }

    pub fn generate_stack(
        &mut self,
        ctx: &mut OmnicomCtx,
        r: &Route,
        gr: &mut GenRoute,
    ) -> Result<(), Box<dyn Error>> {
        let mut hasher = Sha3_256::default();
        let mut actx = AdapterStackCtx::default();

        let mut vars: BiHashMap<String, String> = BiHashMap::new();
        vars.insert("HotContext".into(), "hc".into());
        vars.insert("ColdContext".into(), "cc".into());

        let mut gencode: Vec<u8> = vec![];
        let mut indent = 1;
        let mut emits = vec![];

        let mut middleware_count = 0;

        for c in &r.chain {
            if c.is_middleware() {
                middleware_count += 1;
            }

            let adapter = self
                .adapters
                .iter_mut()
                .find(|a| a.handles(c.get_src_path()))
                .ok_or_else(|| {
                    format!(
                        "Missing LanguageAdaptor for source path: {}",
                        c.get_src_path().display()
                    )
                })?;

            adapter.configure_build(ctx, &mut self.builder);

            indent_fn(indent, &mut gencode)?;
            writeln!(
                gencode,
                "// Generated via {} V{}",
                adapter.get_name(),
                adapter.get_version()
            )?;

            let (i, em) = adapter.emit(ctx, &mut gencode, &mut actx, indent, c)?;
            indent = i;
            hasher.update(em.get_hash());
            emits.push((c, em));
        }

        let stack_hash = encode(hasher.finalize());
        let stack_id = stack_hash[..16].to_string();

        trace!("Generating stack {}", stack_id);

        let mut header = String::new();

        let write_row = |out: &mut String, text: &str| -> Result<(), std::fmt::Error> {
            let mut line = text.to_string();
            if line.chars().count() > 76 {
                line = line.chars().take(73).collect();
                line.push_str("...");
            }
            writeln!(out, "| {:<76} |", line)
        };

        let logo = r#"/*=============================================================================\
|                                                                              |
|      ____                  _   __  __________  ____                          |
|     / __ \____ ___  ____  (_) / / / /_  __/_  __/ __ \                       |
|    / / / / __ `__ \/ __ \/ / / /_/ / / /   / / / /_/ /                       |
|   / /_/ / / / / / / / / / / / __  / / /   / / / ____/                        |
|   \____/_/ /_/ /_/_/ /_/_/ /_/ /_/ /_/   /_/ /_/                             |
|                                                                              |"#;

        writeln!(&mut header, "{}", logo)?;
        write_row(&mut header, "")?;
        writeln!(
            &mut header,
            "|------------------------------------------------------------------------------|"
        )?;

        write_row(
            &mut header,
            &format!("OmniCom V{}", OmnicomCtx::OMNI_VERSION),
        )?;
        write_row(&mut header, &format!("Stack ID:  {}", stack_id))?;
        write_row(
            &mut header,
            &format!("Generated: {}", Utc::now().to_rfc3339()),
        )?;

        let target_triple = env!("BUILD_TARGET");
        write_row(&mut header, &format!("Target: {}", target_triple))?;

        write_row(&mut header, "")?;
        write_row(&mut header, " *** DO NOT MODIFY ***")?;
        write_row(
            &mut header,
            " Auto-generated code. Manual edits will be lost during build.",
        )?;
        write_row(&mut header, "")?;
        writeln!(
            &mut header,
            "|------------------------------------------------------------------------------|"
        )?;

        write_row(
            &mut header,
            &format!("TARGET ROUTE: {} {}", r.method, r.get_path_str()),
        )?;

        write_row(
            &mut header,
            &format!(
                "CHAIN LENGTH: {} ({} Middleware, 1 Endpoint)",
                r.chain.len(),
                middleware_count
            ),
        )?;
        write_row(&mut header, "")?;

        write_row(&mut header, " --- STACK TRACE & NODE DETAILS ---")?;
        for (idx, c) in r.chain.iter().enumerate() {
            let n_type = if c.is_middleware() {
                "MIDDLEWARE"
            } else {
                "ENDPOINT"
            };

            write_row(
                &mut header,
                &format!(
                    " [{:02}] {} -> {}",
                    idx + 1,
                    n_type,
                    c.get_src_path().display()
                ),
            )?;

            let flag_str = if c.get_flags().is_empty() {
                String::from("None")
            } else {
                format!("{:?}", c.get_flags())
            };

            write_row(
                &mut header,
                &format!("      Func:  {} | Flags: {}", c.get_name(), flag_str),
            )?;

            let params = c.get_params();
            if !params.is_empty() {
                let p_str = params
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t))
                    .collect::<Vec<_>>()
                    .join(", ");
                write_row(&mut header, &format!("      Args:  ({})", p_str))?;
            } else {
                write_row(&mut header, "      Args:  None")?;
            }

            let returns = c.get_returns();
            if !returns.is_empty() {
                write_row(&mut header, &format!("      Ret:   {}", returns.join(", ")))?;
            }

            write_row(&mut header, "")?;
        }

        writeln!(
            &mut header,
            "|------------------------------------------------------------------------------|"
        )?;

        write_row(&mut header, " --- GENERATION ARTIFACTS ---")?;

        for (idx, (n, e)) in emits.iter().enumerate() {
            let n_type = if n.is_middleware() { "MID" } else { "END" };

            write_row(
                &mut header,
                &format!(
                    " [{:02}] Layer: {} ({})",
                    idx + 1,
                    n.get_src_path().display(),
                    n_type
                ),
            )?;

            write_row(&mut header, &format!("      Version: {}", e.get_version()))?;
            write_row(&mut header, &format!("    Hash: {}", e.get_hash_str()))?;
            write_row(&mut header, "")?;
        }

        writeln!(
            &mut header,
            "\\=============================================================================*/\n\n\n"
        )?;

        if !actx.ffi.is_empty() {
            writeln!(&mut header, "extern \"C\" {{")?;

            for f in actx.ffi {
                writeln!(&mut header, "    {};", f)?;
            }

            writeln!(&mut header, "}}")?;
        }

        writeln!(&mut header, "\npub fn {}() {{", stack_id)?;
        writeln!(&mut gencode, "}}")?;

        let mut d = ctx.get_bin().to_path_buf();

        d.push(stack_id.clone());

        fs::create_dir_all(&d)?;

        d.push("mod.rs");

        if d.exists() {
            warn!("Overwritting {}", d.display());
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&d)?;

        file.write_all(header.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(&gencode)?;

        gr.get_stack_mut().push_back((r.method, stack_id));
        gr.accept_method(r.method);

        Ok(())
    }
}

impl Default for StackGenerator {
    fn default() -> Self {
        Self {
            builder: OmniBuilder {},
            adapters: vec![],
            pregen_plugins: vec![],
            postgen_plugins: vec![],
        }
    }
}
