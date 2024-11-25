use super::{start_parse, ParsedFile};

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult},
    lexer::Tokens,
    path::Path,
    FILE_EXTENSION,
};
use std::path::PathBuf;

pub fn handle_import(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    project_dir: &PathBuf,
    relative_file_path: Path,
    tokens: &mut Tokens,
) -> CompileResult<(String, ParsedFile)> {
    let from = relative_file_path.clone().pop().unwrap();

    let name = tokens.parse_identifier()?;
    let is_mod_file = from == "mod" ||  (relative_file_path == Path::from("src").join("main") && from == "main");

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

    let mut found_paths: Vec<Path> = Vec::with_capacity(2);
    for path in &paths {
        let mut pathbuf = project_dir.join(path.convert());
        pathbuf.set_extension(FILE_EXTENSION);
        if pathbuf.exists() {
            found_paths.push(path.clone())
        }
    }

    let path = match found_paths.pop() {
        Some(p) => p,
        None => {
            
            return Err(())
            // return Err(DebugInfo::new(
            //     tokens.current().location.clone(),
            //     tokens.relative_file_path.clone(),
            //     format!("Failed to find import path {}, {}", paths[0], paths[1]),
            //     "",
            // ));
        }
    };
    if !found_paths.is_empty() {
        // return Err(DebugInfo::new(
        //     tokens.current().location.clone(),
        //     tokens.relative_file_path.clone(),
        //     format!("Cannot import multiple paths {}, {}", paths[0], paths[1]),
        //     "",
        // ));
        return Err(())
    }

    let mut import = start_parse(debug, count, project_dir, path)?;
    import.is_module = is_mod_file;

    tokens.pop_start();
    return Ok((name, import));
}
