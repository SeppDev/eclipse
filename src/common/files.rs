use crate::compiler::{CompilerCtx, Path};

impl CompilerCtx {
    pub fn fs_read(&self, relative_path: &Path) -> std::io::Result<String> {
        let full_path = self.resolve_path(relative_path);
        std::fs::read_to_string(full_path.as_path_buf())
    }
}
