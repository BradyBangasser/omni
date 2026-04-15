use std::path::{Path, PathBuf};

pub struct OmnicomCtx {
    out: PathBuf,
    bin: PathBuf,
}

impl OmnicomCtx {
    pub const OMNI_VERSION: &'static str = "0.0.1";

    pub fn get_out(&self) -> &Path {
        &self.out
    }

    pub fn get_bin(&self) -> &Path {
        &self.bin
    }
}

impl Default for OmnicomCtx {
    fn default() -> Self {
        Self {
            out: PathBuf::from("./build"),
            bin: PathBuf::from("./build/bin"),
        }
    }
}
