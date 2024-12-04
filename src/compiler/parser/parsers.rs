use std::{collections::BTreeMap, path::PathBuf};

mod arguments;
mod body;
mod enums;
mod expect_token;
mod expression;
mod function;
mod identifier;
mod ifstatement;
mod import;
mod r#loop;
mod namespace;
mod path;
mod types;
mod variable;

use enums::parse_enum;
use function::parse_function;
use import::handle_import;

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult},
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
    pub relative_path: Path,
    pub is_module: bool,
}

pub fn start_parse(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    project_dir: &PathBuf,
    relative_file_path: Path,
    relative_path: Path,
) -> CompileResult<ParsedFile> {
    debug.set_status(format!("Parsing: {}", relative_file_path));

    let mut file_path = project_dir.join(relative_file_path.convert());
    file_path.set_extension(FILE_EXTENSION);
    let source = read_file(&file_path)?;

    debug.set_path(&relative_file_path);

    let mut tokens = tokenize(debug, relative_file_path.clone(), source)?;
    let mut imports = BTreeMap::new();
    let mut body = Vec::new();

    let is_main = relative_file_path == Path::from("src").join("main");

    use super::super::lexer::Token;
    loop {
        if tokens.is_eof() {
            break;
        }

        let info = tokens.expect_tokens(
            vec![
                Token::Import,
                Token::Function,
                Token::Use,
                Token::Enum,
                Token::Struct,
            ],
            true,
        );

        match info.token {
            Token::Import => {
                let (name, import) = match handle_import(
                    debug,
                    count,
                    project_dir,
                    relative_file_path.clone(),
                    &relative_path,
                    &mut tokens,
                ) {
                    Ok(a) => a,
                    Err(()) => continue,
                };
                match imports.insert(name.clone(), import) {
                    Some(_) => {}
                    None => continue,
                };
                return Err(());
            }
            Token::Function => {
                let function = parse_function(&mut tokens, is_main, count, false)?;
                body.push(function)
            }
            Token::Enum => {
                let a = parse_enum(&mut tokens)?;
                body.push(a);
            }
            Token::Struct => todo!(),
            _ => continue,
        }
    }

    tokens.finish(debug);

    let file_name = relative_file_path.clone().pop().unwrap();

    // debug.result_print(format!("\n{:#?}", body));

    let file = ParsedFile {
        imports,
        body,
        is_module: file_name == "mod"
            || (relative_file_path == Path::from("src").join("main") && file_name == "main"),
        relative_file_path,
        relative_path,
    };

    return Ok(file);
}
