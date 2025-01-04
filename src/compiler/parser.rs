use std::collections::BTreeMap;

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

use crate::compiler::nodes::ast::{Node, RawNode};
use crate::compiler::{
    errors::{CompileCtx, CompileResult},
    lexer::{tokenize, Tokens},
    path::Path,
    read_file, FILE_EXTENSION,
};

pub struct ParsedFile {
    pub imports: BTreeMap<String, ParsedFile>,
    pub body: Vec<Node>,
    pub relative_file_path: Path,
    pub is_module: bool,
}

fn handle_tokens(
    ctx: &mut CompileCtx,
    tokens: &mut Tokens,
    imports: &mut BTreeMap<String, ParsedFile>,
    body: &mut Vec<Node>,

    relative_file_path: &Path,
) -> CompileResult<()> {
    let is_main = relative_file_path == &Path::from("src").join("main");
    use super::lexer::Token;

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
                let path = tokens.parse_path()?;
                tokens.create_located(RawNode::NameSpace(path));
            }
            Token::Import => {
                let (name, import) = match tokens.handle_import(ctx, relative_file_path.clone()) {
                    Ok(a) => a,
                    Err(()) => continue,
                };
                match imports.insert(name.clone(), import) {
                    Some(_) => {}
                    None => continue,
                };
            }
            Token::Function => {
                let function = match tokens.parse_function(is_main, ctx.counter.increment()) {
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

pub fn start_parse(ctx: &mut CompileCtx, relative_file_path: Path) -> CompileResult<ParsedFile> {
    ctx.set_status(format!(
        "Parsing: {}.{FILE_EXTENSION}",
        relative_file_path.convert().to_string_lossy()
    ));

    let source = read_file(&ctx.project_dir, &relative_file_path);
    let mut tokens = tokenize(ctx, relative_file_path.clone(), source)?;
    let mut imports: BTreeMap<String, ParsedFile> = BTreeMap::new();
    let mut body: Vec<Node> = Vec::new();

    ctx.set_current_path(&relative_file_path);

    let _ = handle_tokens(
        ctx,
        &mut tokens,
        &mut imports,
        &mut body,
        &relative_file_path,
    );

    tokens.finish(ctx);

    let file_name = relative_file_path.clone().pop().unwrap();

    let file = ParsedFile {
        imports,
        body,
        is_module: file_name == "mod"
            || (relative_file_path == Path::from("src").join("main") && file_name == "main"),
        relative_file_path,
    };

    return Ok(file);
}
