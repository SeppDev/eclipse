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

use crate::compiler::{lexer::tokenize, read_file, FILE_EXTENSION};

use super::NodeInfo;

fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

#[derive(Debug, Default)]
pub struct ParsedFile {
    pub imported: HashMap<String, ParsedFile>,
    pub body: Vec<NodeInfo>,
}
impl ParsedFile {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn parse(project_dir: &PathBuf, relative_path: PathBuf) -> ParsedFile {
    use super::super::lexer::Token;

    let file_path = project_dir.join(&relative_path);
    let source = read_file(&file_path);

    let mut tokens = tokenize(&relative_path, source);
    // println!("{:#?}", tokens);

    let mut file = ParsedFile::new();
    loop {
        if tokens.is_eof() {
            break;
        }

        let info = tokens.expect_tokens(vec![Token::Import, Token::Function], true);

        let node = match info.token {
            Token::Import => {
                let name = tokens.parse_identifer();
                let mut new_path = clean_path(relative_path.parent().unwrap().join(&name));
                new_path.set_extension(FILE_EXTENSION);

                let newfile = parse(project_dir, new_path);
                tokens.pop_start();

                file.imported.insert(name, newfile);
                continue;
            }
            Token::Function => function::parse_function(&mut tokens),
            t => tokens.throw_error(format!("Expected item, found '{}'", t), ""),
        };
        file.body.push(node);
    }
    tokens.finish();

    return file;
}
