use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

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
    counter::NameCounter,
    errors::{CompileMessages, Location, Message, MessageKind},
    lexer::tokenize,
    read_file, FILE_EXTENSION,
};
use namespace::parse_namespace;

use super::NodeInfo;

fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

#[derive(Debug, Default)]
pub struct ParsedFile {
    pub export: bool,
    pub relative_path: PathBuf,
    pub imported: HashMap<String, ParsedFile>,
    pub functions: BTreeMap<String, NodeInfo>,
    pub lines: Vec<String>,
    pub errors: CompileMessages,
}
impl ParsedFile {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn throw_error<T: ToString, E: ToString>(
        &mut self,
        message: T,
        notice: E,
        location: &Location,
    ) -> &mut Message {
        self.errors.create(
            MessageKind::Error,
            self.relative_path.clone(),
            message,
            notice,
            location.clone(),
        )
    }
}

pub fn parse(
    counter: &mut NameCounter,
    project_dir: &PathBuf,
    mut relative_path: PathBuf,
    source: String,
) -> ParsedFile {
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
                let mut new_relative_path = relative_path.parent().unwrap().join(&name);
                new_relative_path.set_extension(FILE_EXTENSION);

                let source = read_file(&project_dir.join(&new_relative_path));
                let mut newfile = parse(counter, project_dir, new_relative_path, source);
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
                tokens.throw_error(format!("Expected item, found '{}'", t), "", info.location.clone());
                continue;
            }
        };
    }
    let lines = tokens.finish();
    file.relative_path = relative_path;
    file.lines = lines;

    return file;
}
