use std::path::PathBuf;

use crate::path::Path;

pub trait ResolveModule {
    fn resolve_module(&self, path: &PathBuf) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    fn project_path(&self) -> &Path;
}

pub struct ModuleResolver {
    project_path: Path,
}
impl ModuleResolver {
    pub fn new(project_path: Path) -> Self {
        Self {
            project_path
        }
    }
}
impl ResolveModule for ModuleResolver {
    fn project_path(&self) -> &Path {
        &self.project_path
    }
}
