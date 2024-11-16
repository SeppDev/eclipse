use std::path::PathBuf;
use crate::compiler::{errors::CompileMessages, lexer::Tokens, path::Path};
use super::{start_parse, ParsedFile};

pub fn handle_import(compile_messages: &mut CompileMessages, project_dir: &PathBuf, relative_path: &Path, tokens: &mut Tokens) -> (String, ParsedFile) {
    let name = tokens.parse_identifier().unwrap();
    let import = start_parse(
        compile_messages,
        project_dir,
        relative_path.parent().join(&name),
    );
    tokens.pop_start();
    return (name, import);
}
