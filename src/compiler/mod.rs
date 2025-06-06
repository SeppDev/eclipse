use context::status::Status;
use diagnostics::Diagnostics;

use crate::FILE_EXTENSION;

pub type Path = crate::common::path::Path;

pub mod analyzer;
pub mod ast;
pub mod common;
pub mod context;
pub mod diagnostics;
pub mod lexer;
pub mod parser;

pub struct CompilerBuilder {
    status: bool,
    project_path: Path,
}
impl CompilerBuilder {
    pub fn new() -> Self {
        Self {
            status: true,
            project_path: Path::default(),
        }
    }
    pub fn status(mut self, enabled: bool) -> Self {
        self.status = enabled;
        self
    }
    pub fn project_path(mut self, path: Path) -> Self {
        self.project_path = path;
        self
    }
    pub fn build(self) -> CompilerCtx {
        let project_path = self.project_path;
        let mut path = Path::single("std");
        path.set_extension(FILE_EXTENSION);

        CompilerCtx {
            project_path,
            logs: Vec::new(),
            diagnostics: Diagnostics::new(),
            status: self.status.then(|| Status::new()),
        }
    }
}

pub struct CompilerCtx {
    status: Option<Status>,
    project_path: Path,
    diagnostics: Diagnostics,
    logs: Vec<String>,
}
impl CompilerCtx {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::new()
    }
    pub fn log(&mut self, message: impl ToString) {
        self.logs.push(message.to_string());
    }
    pub fn resolve_path(&self, relative_path: &Path) -> Path {
        self.project_path.clone().extend(relative_path)
    }
    pub fn check_diagnostics(&self) {
        if !self.diagnostics.has_errors() {
            return;
        }
        self.diagnostics.display();
        std::process::exit(0)
    }
    pub fn finish(self) {
        if let Some(status) = &self.status {
            status.quit();
        }
        self.check_diagnostics();

        println!(
            "{}",
            self.logs
                .into_iter()
                .map(|msg| format!("[LOG]: {msg}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
