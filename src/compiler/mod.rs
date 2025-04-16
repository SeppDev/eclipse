use context::config::Config;
use status::Status;
use std::path::PathBuf;

use crate::{
    cli::options::CommandLineOptions,
    common::files::Files,
    diagnostics::{DiagnosticData, DiagnosticResult, Diagnostics},
};

pub mod analyzer;
pub mod context;
pub mod lexer;
pub mod nodes;
pub mod parser;
pub mod status;

pub struct CompilerCtx {
    status: Option<Status>,
    config: Config,
    project_path: PathBuf,
    files: Files,
    diagnostics: Diagnostics,
}
impl CompilerCtx {
    pub fn new(options: CommandLineOptions) -> DiagnosticResult<Self> {
        let config = Config::open(&options.active_path)?;

        Ok(Self {
            config,
            diagnostics: Diagnostics::new(),
            status: options.status.then(|| Status::new()),
            project_path: options.active_path,
            files: Files::new(),
        })
    }
    fn collect_error<T>(&mut self, result: DiagnosticResult<T>) -> Result<T, ()> {
        self.diagnostics.collect_error(result)
    }
    fn read_relative(&self, relative_path: &PathBuf) -> DiagnosticResult<String> {
        let path = self.project_path.join(relative_path);
        match self.files.cache_or_read(&path) {
            Some(s) => Ok(s),
            None => Err(DiagnosticData::basic(format!("Path not found"), path)),
        }
    }
    fn read(&self, full_path: &PathBuf) -> DiagnosticResult<String> {
        match self.files.cache_or_read(&full_path) {
            Some(s) => Ok(s),
            None => Err(DiagnosticData::basic(
                format!("Path not found"),
                full_path.clone(),
            )),
        }
    }
}
