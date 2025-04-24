use std::f32::consts::PI;

use context::status::Status;
use diagnostics::Diagnostics;

use crate::common::files::Files;

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
    pub fn disable_std(mut self) -> Self {
        todo!()
    }
    pub fn project_path(mut self, path: Path) -> Self {
        self.project_path = path;
        self
    }
    pub fn build(self) -> CompilerCtx {
        let project_path = self.project_path;
        CompilerCtx {
            project_path,
            diagnostics: Diagnostics::new(),
            status: self.status.then(|| Status::new()),
            files: Files::new(),
        }
    }
}

pub struct CompilerCtx {
    pub status: Option<Status>,
    pub files: Files,
    project_path: Path,
    diagnostics: Diagnostics,
}
impl Drop for CompilerCtx {
    fn drop(&mut self) {
        if let Some(status) = &self.status {
            status.quit();
        }
    }
}
impl CompilerCtx {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::new()
    }
    pub fn resolve_path(&self, relative_path: Path) -> Path {
        let mut path = self.project_path.clone();
        if let Some(ext) = relative_path.extension() {
            path.set_extension(ext);
        }
        path.extend(relative_path);
        path
    }
    pub fn then<F>(self, func: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        func(self)
    }
}
