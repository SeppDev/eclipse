use std::{collections::BTreeMap, path::PathBuf};

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

use crate::compiler::{
    counter::NameCounter, errors::CompileMessages, lexer::tokenize, path::Path, read_file,
    FILE_EXTENSION,
};
use namespace::parse_namespace;

use super::NodeInfo;

#[derive(Debug)]
pub struct ParsedFile {
    pub export: bool,
    pub relative_path: Path,
    pub imported: BTreeMap<String, ParsedFile>,
    pub functions: BTreeMap<String, NodeInfo>,
    pub lines: Vec<String>,
}
impl ParsedFile {
    pub fn new() -> Self {
        Self {

        }
    }
}

pub fn parse(
    counter: &mut NameCounter,
    errors: &mut CompileMessages,
    project_dir: &PathBuf,
    mut relative_path: Path,
    source: String,
) -> ParsedFile {
    use super::super::lexer::Token;

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
                let mut new_relative_path = relative_path.parent().unwrap().join(&name);
                new_relative_path.set_extension(FILE_EXTENSION);

                let source = read_file(&project_dir.join(&new_relative_path));
                let mut newfile = parse(counter, errors, project_dir, new_relative_path, source);
                tokens.pop_start();
                newfile.export = public;

                file.imported.insert(name, newfile);
                continue;
            }
            Token::Function => {
                let name = tokens.parse_identifer();
                let function = function::parse_function(counter, &mut tokens, public);

                match file.functions.remove(&name) {
                    Some(old) => {
                        let message = tokens.throw_error(
                            format!("There's already a function named '{}'", name),
                            "",
                            old.location,
                        );
                        message.push("", function.location.clone());
                    }
                    None => {}
                }

                file.functions.insert(name.clone(), function).unwrap();
                continue;
            }
            t => {
                tokens.throw_error(
                    format!("Expected item, found '{}'", t),
                    "",
                    info.location.clone(),
                );
                continue;
            }
        };
    }
    let lines = tokens.finish();
    file.relative_path = relative_path;
    file.lines = lines;

    return file;
}
