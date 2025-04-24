use context::status::Status;
use diagnostics::{DiagnosticResult, Diagnostics};

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
}
impl CompilerBuilder {
    pub fn new() -> Self {
        Self { status: true }
    }
    pub fn status(mut self, enabled: bool) -> Self {
        self.status = enabled;
        self
    }
    pub fn build(self) -> CompilerCtx {
        CompilerCtx {
            diagnostics: Diagnostics::new(),
            status: self.status.then(|| Status::new()),
            files: Files::new(),
        }
    }
}

pub struct CompilerCtx {
    pub status: Option<Status>,
    pub files: Files,
    diagnostics: Diagnostics,
}
impl CompilerCtx {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::new()
    }
    pub fn test() -> Self {
        Self::builder().status(false).build()
    }
    pub fn then<F>(self, func: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        func(self)
    }
}
