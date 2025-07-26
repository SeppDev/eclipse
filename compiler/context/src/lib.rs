use std::path::PathBuf;

use common::{constants::FILE_EXTENSION, status::Status};
use diagnostics::Diagnostics;
use files::{FileResolver, ResolveFile};

pub mod files;

#[derive(Default)]
pub struct CompilerBuilder {
    status: bool,
    project_path: Option<PathBuf>,
    module_resolver: FileResolver,
}
impl CompilerBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn resolver(mut self, resolver: impl Into<FileResolver>) -> Self {
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

#[allow(non_camel_case_types)]
pub enum Arch {
    x86_64,
    x86,
    arm_64,
    arm,
}

pub struct CompilerCtx {
    status: Option<Status>,
    module_resolver: FileResolver,
    project_path: PathBuf,
    logs: Vec<String>,
    pub diagnostics: Diagnostics,
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
    pub fn resolve_path(&self, relative_path: &PathBuf) -> PathBuf {
        self.project_path.join(relative_path)
    }
    pub fn entry() -> PathBuf {
        let src_path = PathBuf::from("src");
        let mut main_path = src_path.join("main");
        main_path.set_extension(FILE_EXTENSION);
        main_path
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
