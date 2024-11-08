use std::{collections::HashMap, path::PathBuf};

mod arguments;
mod body;
mod expect_token;
mod expression;
mod function;
mod identifier;
mod path;
mod types;
mod variable;
mod namespace;
// mod dot;

use namespace::parse_namespace;

use crate::compiler::{lexer::tokenize, read_file, FILE_EXTENSION};

use super::NodeInfo;

fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

#[derive(Debug, Default)]
pub struct ParsedFile {
    pub export: bool,
    pub imported: HashMap<String, ParsedFile>,
    pub body: Vec<NodeInfo>,
}
impl ParsedFile {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn parse(project_dir: &PathBuf, mut relative_path: PathBuf, source: String) -> ParsedFile {
    use super::super::lexer::Token;

    // let file_path = project_dir.join(&relative_path);
    relative_path = clean_path(relative_path);

    let mut tokens = tokenize(&relative_path, source);
    // println!("{:#?}", tokens);

    let mut file = ParsedFile::new();
    loop {
        if tokens.is_eof() {
            break;
        }

        let public = tokens.peek_expect_token(Token::Pub, true);
        let info = tokens.expect_tokens(vec![Token::Import, Token::Function, Token::Use], true);

        let node = match info.token {
            Token::Import => {
                let name = tokens.parse_identifer();
                let mut new_path = relative_path.parent().unwrap().join(&name);
                new_path.set_extension(FILE_EXTENSION);

                let source = read_file(&new_path);
                let mut newfile = parse(project_dir, new_path, source);
                tokens.pop_start();
                newfile.export = public;

                file.imported.insert(name, newfile);
                continue;
            }
            Token::Use => parse_namespace(&mut tokens, public),
            Token::Function => function::parse_function(&mut tokens, public),
            t => tokens.throw_error(format!("Expected item, found '{}'", t), ""),
        };
        file.body.push(node);
    }
    tokens.finish();

    return file;
}
