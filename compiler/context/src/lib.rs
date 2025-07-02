use common::status::Status;
use diagnostics::Diagnostics;
use resolver::{FileSystemResolver, ModuleResolver};

pub type Path = common::path::Path;

mod resolver;

pub struct CompilerBuilder<Resolver: ModuleResolver = FileSystemResolver> {
    status: bool,
    project_path: Option<Path>,
    module_resolver: Option<Resolver>,
}
impl<Resolver: ModuleResolver> CompilerBuilder<Resolver> {
    pub fn new() -> Self {
        Self {
            status: false,
            module_resolver: None,
            project_path: None,
        }
    }
    pub fn resolver(mut self, resolver: Resolver) -> Self {
        self.module_resolver = Some(resolver);
        self
    }
    pub fn status(mut self, enabled: bool) -> Self {
        self.status = enabled;
        self
    }
    pub fn project_path(mut self, path: Path) -> Self {
        self.project_path = Some(path);
        self
    }
    pub fn build(self) -> CompilerCtx {
        let project_path = self.project_path.expect("Expected a project path");

        CompilerCtx {
            logs: Vec::new(),
            project_path,
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
    pub fn builder<T: ModuleResolver>() -> CompilerBuilder<T> {
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
