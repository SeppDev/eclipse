use std::path::PathBuf;

mod arguments;
mod body;
mod expect_token;
mod expression;
mod function;
mod identifier;
mod ifstatement;
mod import;
mod namespace;
mod path;
mod types;
mod variable;

use function::parse_function;
use import::handle_import;

use crate::compiler::{
    errors::{CompileMessages, CompileResult},
    lexer::tokenize,
    path::Path,
    read_file, FILE_EXTENSION,
};

use super::NodeInfo;

#[derive(Debug)]
pub struct ParsedFile {
    pub imports: Vec<(String, ParsedFile)>,
    pub body: Vec<NodeInfo>,
    pub relative_file_path: Path,
}

pub fn start_parse(
    compile_messages: &mut CompileMessages,
    project_dir: &PathBuf,
    relative_file_path: Path,
) -> CompileResult<ParsedFile> {
    let mut file_path = project_dir.join(relative_file_path.convert());
    file_path.set_extension(FILE_EXTENSION);

    let source = read_file(&file_path);

    let mut tokens = tokenize(compile_messages, relative_file_path.clone(), source)?;
    
    let mut imports = Vec::new();
    let mut body = Vec::new();

    use super::super::lexer::Token;
    loop {
        if tokens.is_eof() {
            break;
        }

        let info = tokens.expect_tokens(vec![Token::Import, Token::Function, Token::Use], true)?;

        match info.token {
            Token::Import => {
                let (name, import) = handle_import(
                    compile_messages,
                    project_dir,
                    relative_file_path.clone(),
                    &mut tokens,
                )?;
                imports.push((name, import));
            }
            Token::Function => {
                let function = match parse_function(&mut tokens, false) {
                    Ok(f) => f,
                    Err(()) => break,
                };
                body.push(function)
            }
            Token::Enum => todo!(),
            Token::Struct => todo!(),
            _ => continue,
        }
    }

    tokens.finish(compile_messages);

    let file = ParsedFile {
        imports,
        body,
        relative_file_path,
    };

    return Ok(file);
}
