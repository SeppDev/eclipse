use std::path::PathBuf;

use common::status::Status;
use diagnostics::Diagnostics;
use resolver::{ModuleResolver, Resolver};

pub mod resolver;

pub struct CompilerBuilder {
    status: bool,
    project_path: Option<PathBuf>,
    module_resolver: Resolver,
}
impl CompilerBuilder {
    pub fn new() -> Self {
        Self {
            status: false,
            module_resolver: Resolver::default(),
            project_path: None,
        }
    }
    pub fn resolver(mut self, resolver: impl Into<Resolver>) -> Self {
        self.module_resolver = resolver.into();
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
        let module_resolver = self.module_resolver;

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
    pub diagnostics: Diagnostics,
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
        self.module_resolver.read(&path)
    }
    pub fn write(&mut self, relative_path: &PathBuf, contents: &str) {
        let path = self.resolve_path(relative_path);
        self.module_resolver.write(&path, contents);
    }
    pub fn log(&mut self, message: impl ToString) {
        self.logs.push(message.to_string());
    }
    #[inline]
    pub fn resolve_path(&self, relative_path: &PathBuf) -> PathBuf {
        self.project_path.join(relative_path)
    }
    pub fn finish(self) {
        if let Some(status) = &self.status {
            status.quit();
        }

        println!(
            "{}",
            self.logs
                .into_iter()
                .map(|msg| format!("[LOG]: {msg}"))
                .collect::<Vec<String>>()
                .join("\n")
        );

        self.diagnostics.check();
    }
}
