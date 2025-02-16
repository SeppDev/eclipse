use std::path::PathBuf;
use context::config::Config;
use status::Status;

use crate::{cli::options::CommandLineOptions, common::files::Files};

pub mod analyzer;
pub mod lexer;
pub mod parser;
pub mod status;
pub mod context;

pub struct CompilerCtx {
    status: Option<Status>,
    config: Config,
    files: Files,
}
impl CompilerCtx {
    pub fn new(options: CommandLineOptions) -> Self {
        Self {
            status: options.status.then(|| Status::new()),
            config: Config::default(),
            files: Files::new()
        }
    }
    fn read_or_cache(&mut self, full_path: &PathBuf) -> Option<&String> {
        self.files.read_or_cache(full_path)
    }
    fn from_cache(&self, full_path: &PathBuf) -> Option<&String> {
        self.files.from_cache(full_path)
    }
}
