use std::path::PathBuf;

use common::status::Status;
use diagnostics::Diagnostics;
use resolver::{ModuleResolver, Resolver};

pub mod resolver;

pub struct CompilerBuilder {
    status: bool,
    project_path: Option<PathBuf>,
    module_resolver: Option<Resolver>,
}
impl CompilerBuilder {
    pub fn new() -> Self {
        Self {
            status: false,
            module_resolver: None,
            project_path: None,
        }
    }
    pub fn resolver(mut self, resolver: impl Into<Resolver>) -> Self {
        self.module_resolver = Some(resolver.into());
        self
    }
    pub fn status(mut self, enabled: bool) -> Self {
        self.status = enabled;
        self
    }
    pub fn project_path(mut self, path: PathBuf) -> Self {
        self.project_path = Some(path);
        self
    }
    pub fn build(self) -> CompilerCtx {
        let project_path = self.project_path.expect("Expected a project path");
        let module_resolver = self.module_resolver.expect("Expected a module resolver");

        CompilerCtx {
            logs: Vec::new(),
            module_resolver,
            project_path,
            diagnostics: Diagnostics::new(),
            status: self.status.then(|| Status::new()),
        }
    }
}

pub struct CompilerCtx {
    status: Option<Status>,
    module_resolver: Resolver,
    project_path: PathBuf,
    diagnostics: Diagnostics,
    logs: Vec<String>,
}
impl CompilerCtx {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::new()
    }
    pub fn status(&self, message: String) {
        let status = match &self.status {
            Some(s) => s,
            None => return,
        };
        status.message(message);
    }
    pub fn read(&self, relative_path: &PathBuf) -> Option<String> {
        let path = self.resolve_path(relative_path);
        self.module_resolver.resolve_module(&path)
    }
    pub fn log(&mut self, message: impl ToString) {
        self.logs.push(message.to_string());
    }
    #[inline]
    pub fn resolve_path(&self, relative_path: &PathBuf) -> PathBuf {
        self.project_path.join(relative_path)
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
