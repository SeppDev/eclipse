use crate::{
    common::path::Path,
    compiler::{
        diagnostics::{DiagnosticData, DiagnosticResult},
        CompilerCtx,
    },
    FILE_EXTENSION,
};

impl CompilerCtx {
    pub fn handle_import(&self, current_relative_path: &Path, name: &str) -> DiagnosticResult<Path> {
        let file_name = current_relative_path.last().unwrap();
        let is_module = file_name == "mod" || file_name == "main";

        let parent = current_relative_path.parent();
        let expected_paths: [Path; 2] = if is_module {
            [parent.join(&name), parent.join(&name).join("mod")]
        } else {
            [
                parent.join(&file_name).join(&name),
                parent.join(&name).join(&file_name).join("mod"),
            ]
        };

        let mut found: Vec<Path> = Vec::with_capacity(2);
        for relative_path in &expected_paths {
            let mut relative_path = relative_path.to_owned();
            relative_path.set_extension(FILE_EXTENSION);

            let full_path = self.resolve_path(relative_path.clone());
            if full_path.exists() {
                found.push(relative_path);
            }
        }

        if found.len() > 1 {
            return DiagnosticData::error()
                .title(format!(
                    "Unresolved module, found two modules {expected_paths:?}"
                ))
                .to_err();
        }

        if let Some(path) = found.pop() {
            return Ok(path);
        }

        DiagnosticData::error()
            .title(format!(
                "Unresolved module, can't find module {expected_paths:?}"
            ))
            .to_err()
    }
}
