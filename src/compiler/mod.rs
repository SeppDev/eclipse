use std::path::PathBuf;

use context::status::Status;
use diagnostics::Diagnostics;

use crate::{common::files::Files, FILE_EXTENSION};

pub type Path = crate::common::path::Path;

pub mod analyzer;
pub mod context;
pub mod diagnostics;
pub mod lexer;
pub mod nodes;
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
        let mut files = Files::new();
        let mut path = Path::single("std");
        path.set_extension(FILE_EXTENSION);

        files.cache(path, "import test".to_string());

        CompilerCtx {
            project_path,
            files,
            diagnostics: Diagnostics::new(),
            status: self.status.then(|| Status::new()),
        }
    }
}

#[allow(unused)]
pub struct CompilerCtx {
    pub files: Files,
    status: Option<Status>,
    project_path: Path,
    diagnostics: Diagnostics,
}
impl CompilerCtx {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::new()
    }
    pub fn resolve_path(&self, relative_path: Path) -> Path {
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
        self.check_diagnostics();
        if let Some(status) = self.status {
            status.quit();
        }
    }
}
