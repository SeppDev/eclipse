use super::{start_parse, ParsedFile};

use crate::compiler::{
    errors::{CompileMessages, CompileResult, MessageKind},
    lexer::Tokens,
    path::Path,
    FILE_EXTENSION,
};
use std::path::PathBuf;

pub fn handle_import(
    compile_messages: &mut CompileMessages,
    project_dir: &PathBuf,
    relative_file_path: Path,
    tokens: &mut Tokens,
) -> CompileResult<(String, ParsedFile)> {
    let from = relative_file_path.clone().pop().unwrap();
    let is_mod_file =
        from == "mod" || (relative_file_path == Path::from("src").join("main") && from == "main");

    let name = tokens.parse_identifier()?;

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
            compile_messages.create(
                MessageKind::Error,
                tokens.current().location.clone(),
                relative_file_path,
                format!("Failed to find import path {}, {}", paths[0], paths[1]),
                "",
            );
            return Err(());
        }
    };
    if !found_paths.is_empty() {
        compile_messages.create(
            MessageKind::Error,
            tokens.current().location.clone(),
            relative_file_path,
            format!("Cannot import multiple paths {}, {}", paths[0], paths[1]),
            "",
        );
        return Err(());
    }

    

    let mut import = start_parse(compile_messages, project_dir, path)?;
    import.is_module = is_mod_file;

    println!("{:?}", import);

    tokens.pop_start();
    return Ok((name, import));
}
