
use std::{collections::HashMap, path::PathBuf};

mod arguments;
mod body;
mod expect_token;
mod expression;
mod function;
mod identifier;
mod namespace;
mod path;
mod types;
mod variable;
// mod dot;

use namespace::parse_namespace;
use crate::compiler::{errors::throw_error, lexer::{tokenize, Location}, read_file, FILE_EXTENSION};
use super::Function;

fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

#[derive(Debug, Default)]
pub struct ParsedFile {
    pub export: bool,
    pub relative_path: PathBuf,
    pub imported: HashMap<String, ParsedFile>,
    pub functions: HashMap<String, Function>,
    pub lines: Vec<String>,
}
impl ParsedFile {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn throw_error<T: ToString>(&self, message: T, location: &Location) -> ! {
        throw_error(message, &self.relative_path, location, &self.lines)
    }
}

pub fn parse(project_dir: &PathBuf, mut relative_path: PathBuf, source: String) -> ParsedFile {
    use super::super::lexer::Token;

    relative_path = clean_path(relative_path);
    let mut tokens = tokenize(&relative_path, source);
    let mut file = ParsedFile::new();
    loop {
        if tokens.is_eof() {
            break;
        }

        let public = tokens.peek_expect_tokens(vec![Token::Pub], true).is_some();
        let info = tokens.expect_tokens(vec![Token::Import, Token::Function, Token::Use], true);

        match info.token {
            Token::Use => parse_namespace(&mut tokens, public),
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
            Token::Function => {
                let name = tokens.parse_identifer();
                let function = function::parse_function(&mut tokens, public);
                match file.functions.insert(name.clone(), function) {
                    Some(_) => panic!("There's already a function named '{}'", name),
                    None => {}
                };
                continue;
            }
            t => tokens.throw_error(format!("Expected item, found '{}'", t), ""),
        };
    }
    let lines = tokens.finish();
    file.lines = lines;
    file.relative_path = relative_path;

    return file;
}
