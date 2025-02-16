use std::path::PathBuf;
use context::config::Config;

use crate::common::files::Files;

pub mod analyzer;
pub mod lexer;
pub mod parser;
pub mod status;
pub mod context;

pub struct CompilerCtx {
    status: Option<status::Status>,
    config: Config,
    files: Files,
}
impl CompilerCtx {
    fn read_or_cache(&mut self, full_path: &PathBuf) -> Option<&String> {
        self.files.read_or_cache(full_path)
    }
    fn from_cache(&self, full_path: &PathBuf) -> Option<&String> {
        self.files.from_cache(full_path)
    }
}
