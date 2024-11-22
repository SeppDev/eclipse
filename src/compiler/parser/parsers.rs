use std::{collections::BTreeMap, path::PathBuf};

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
    counter::NameCounter,
    errors::{CompileMessages, CompileResult, DebugInfo},
    lexer::tokenize,
    path::Path,
    read_file, FILE_EXTENSION,
};

use super::NodeInfo;

#[derive(Debug)]
pub struct ParsedFile {
    pub imports: BTreeMap<String, ParsedFile>,
    pub body: Vec<NodeInfo>,
    pub relative_file_path: Path,
    pub is_module: bool,
}

pub fn start_parse(
    compile_messages: &mut CompileMessages,
    name_counter: &mut NameCounter,
    project_dir: &PathBuf,
    relative_file_path: Path,
) -> CompileResult<ParsedFile> {
    let mut file_path = project_dir.join(relative_file_path.convert());
    file_path.set_extension(FILE_EXTENSION);
    let source = read_file(&file_path)?;

    let mut tokens = tokenize(compile_messages, relative_file_path.clone(), source)?;
    let mut imports = BTreeMap::new();
    let mut body = Vec::new();

    let is_main = relative_file_path == Path::from("src").join("main");

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
                    name_counter,
                    project_dir,
                    relative_file_path.clone(),
                    &mut tokens,
                )?;
                match imports.insert(name.clone(), import) {
                    Some(_) => {}
                    None => continue,
                };
                return Err(DebugInfo::new(
                    info.location,
                    relative_file_path,
                    format!("There is already an import named: '{name}'"),
                    "",
                ));
            }
            Token::Function => {
                let function = parse_function(&mut tokens, is_main, name_counter, false)?;
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
        is_module: false,
    };

    return Ok(file);
}
