use super::{start_parse, ParsedFile};

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult},
    file_exists,
    lexer::Tokens,
    path::Path,
};
use std::path::PathBuf;

impl Tokens {
    pub fn handle_import(
        &mut self,
        debug: &mut CompileCtx,
        count: &mut NameCounter,
        project_dir: &PathBuf,
        relative_file_path: Path,
        relative_path: &Path,
    ) -> CompileResult<(String, ParsedFile)> {
        let from = relative_file_path.clone().pop().unwrap();

        let name = self.parse_identifier()?;
        let is_mod_file = from == "mod"
            || (relative_file_path == Path::from("src").join("main") && from == "main");

        let paths: [Path; 2] = if is_mod_file {
            [
                relative_file_path.parent().join(&name),
                relative_file_path.parent().join(&name).join("mod"),
            ]
        } else {
            [
                relative_file_path.parent().join(&from).join(&name),
                relative_file_path
                    .parent()
                    .join(&name)
                    .join(&from)
                    .join("mod"),
            ]
        };

        let failed_to_find = format!("Failed to find import path {}, {}", paths[0], paths[1]);
        let failed_multiple = format!("Cannot import multiple paths {}, {}", paths[0], paths[1]);

        let mut found_paths: Vec<Path> = Vec::with_capacity(2);
        for path in paths {
            if file_exists(project_dir, &path) {
                found_paths.push(path)
            }
        }

        let path = match found_paths.pop() {
            Some(p) => p,
            None => {
                self.error(self.current().location.clone(), failed_to_find);
                return Err(());
            }
        };
        if !found_paths.is_empty() {
            self.error(self.current().location.clone(), failed_multiple);
            return Err(());
        }

        let mut import = start_parse(debug, count, project_dir, path, relative_path.join(&name))?;
        import.is_module = is_mod_file;

        self.pop_start();
        return Ok((name, import));
    }
}
