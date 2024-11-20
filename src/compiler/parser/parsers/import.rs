use super::{start_parse, ParsedFile};
use crate::compiler::{
    errors::{CompileMessages, CompileResult},
    lexer::Tokens,
    path::Path,
};
use std::path::PathBuf;

pub fn handle_import(
    compile_messages: &mut CompileMessages,
    project_dir: &PathBuf,
    relative_file_path: Path,
    tokens: &mut Tokens,
) -> CompileResult<(String, ParsedFile)> {
    let from = relative_file_path.clone().pop().unwrap();
    println!("{}", from);

    let name = tokens.parse_identifier()?;
    let import = start_parse(
        compile_messages,
        project_dir,
        relative_file_path.parent().join(&name)
    )?;
    tokens.pop_start();
    return Ok((name, import));
}
