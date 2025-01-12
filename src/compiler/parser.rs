use std::collections::BTreeMap;

mod arguments;
mod body;
mod enums;
mod expect_token;
mod expression;
mod fields;
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

use super::nodes::ast::{Function, Layout};

#[derive(Debug)]
pub struct ParsedFile {
    pub relative_file_path: Path,
    pub imports: BTreeMap<String, ParsedFile>,
    pub layouts: Vec<Layout>,
    pub functions: Vec<Function>,
}

fn handle_tokens(
    ctx: &mut CompileCtx,
    tokens: &mut Tokens,
    file: &mut ParsedFile,
) -> CompileResult<()> {
    let is_main = &file.relative_file_path == &Path::from("src").join("main");
    use super::lexer::Token;

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
        Token::Use => todo!(),
        Token::Import => {
            let (name, import) = tokens.handle_import(ctx, file.relative_file_path.clone())?;
            match file.imports.insert(name, import) {
                Some(_) => {}
                None => return Err(()),
            };
        }
        Token::Function => {
            let function =
                tokens.parse_function(is_main, ctx.counter.increment())?;
            file.functions.push(function)
        }
        Token::Enum => file.layouts.push(tokens.parse_enum()?),
        Token::Struct => file.layouts.push(tokens.parse_struct()?),
        _ => {}
    }

    return Ok(());
}

pub fn start_parse(ctx: &mut CompileCtx, relative_file_path: Path) -> CompileResult<ParsedFile> {
    ctx.set_current_path(relative_file_path);
    ctx.set_status(format!(
        "Parsing: {}.{FILE_EXTENSION}",
        ctx.current_file_path.into_path_buf().to_string_lossy()
    ));

    let source = read_file(&ctx.project_dir, &ctx.current_file_path);
    let mut tokens = tokenize(ctx, source)?;


    let mut file = ParsedFile {
        relative_file_path: ctx.current_file_path.clone(),
        imports: BTreeMap::new(),
        layouts: Vec::new(),
        functions: Vec::new(),
    };

    loop {
        if tokens.is_eof() {
            break;
        }
        let _ = handle_tokens(ctx, &mut tokens, &mut file);
    }

    tokens.finish(ctx);

    return Ok(file);
}
