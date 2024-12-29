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
mod loops;
mod namespace;
mod path;
mod structs;
mod types;
mod variable;

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult},
    lexer::{tokenize, Tokens},
    parser::Node,
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

fn handle_tokens(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    project_dir: &PathBuf,
    tokens: &mut Tokens,
    imports: &mut BTreeMap<String, ParsedFile>,
    body: &mut Vec<NodeInfo>,

    relative_file_path: &Path,
    relative_path: &Path,
) -> CompileResult<()> {
    let is_main = relative_file_path == &Path::from("src").join("main");

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
        )?;

        match info.token {
            Token::Use => {
                let root = tokens.parse_identifier()?;
                let path = tokens.parse_path(&root)?;
                tokens.create_node(Node::NameSpace {
                    public: false,
                    static_path: path,
                });
            }
            Token::Import => {
                let (name, import) = match tokens.handle_import(
                    debug,
                    count,
                    project_dir,
                    relative_file_path.clone(),
                    &relative_path,
                ) {
                    Ok(a) => a,
                    Err(()) => continue,
                };
                match imports.insert(name.clone(), import) {
                    Some(_) => {}
                    None => continue,
                };
            }
            Token::Function => {
                let function = match tokens.parse_function(is_main, count, false) {
                    Ok(f) => f,
                    Err(()) => continue,
                };
                body.push(function)
            }
            Token::Enum => body.push(tokens.parse_enum()?),
            Token::Struct => body.push(tokens.parse_struct()?),
            _ => continue,
        }
    }
    return Ok(());
}

pub fn start_parse(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    project_dir: &PathBuf,
    relative_file_path: Path,
    relative_path: Path,
) -> CompileResult<ParsedFile> {
    debug.set_status(format!(
        "Parsing: {}.{FILE_EXTENSION}",
        relative_file_path.convert().to_string_lossy()
    ));

    let source = read_file(project_dir, &relative_file_path);
    let mut tokens = tokenize(debug, relative_file_path.clone(), source)?;
    let mut imports: BTreeMap<String, ParsedFile> = BTreeMap::new();
    let mut body: Vec<NodeInfo> = Vec::new();

    debug.set_path(&relative_file_path);

    let _ = handle_tokens(
        debug,
        count,
        project_dir,
        &mut tokens,
        &mut imports,
        &mut body,
        &relative_file_path,
        &relative_path,
    );

    tokens.finish(debug);

    let file_name = relative_file_path.clone().pop().unwrap();

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
