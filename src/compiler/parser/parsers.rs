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
    counter::NameCounter,
    errors::{CompileMessages, FileMessages, MessageKind},
    lexer::tokenize,
    path::Path,
    read_file, FILE_EXTENSION,
};
use namespace::parse_namespace;

use super::NodeInfo;

#[derive(Debug, Default)]
pub struct ParsedFile {
    pub export: bool,
    pub imported: BTreeMap<String, ParsedFile>,
    pub functions: BTreeMap<String, NodeInfo>,
    pub messages: FileMessages
}
impl ParsedFile {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn parse(
    counter: &mut NameCounter,
    messages: &mut CompileMessages,
    project_dir: &PathBuf,
    relative_path: Path,
    source: String,
) -> ParsedFile {
    use super::super::lexer::Token;

    let mut tokens = tokenize(messages, source);
    let mut parsed_file = ParsedFile::new();

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
                let relative_path = relative_path.parent().join(&name);

                let mut file_path = project_dir.join(relative_path.convert());
                file_path.set_extension(FILE_EXTENSION);

                let source = read_file(&file_path);
                let mut newfile = parse(counter, messages, project_dir, relative_path, source);
                tokens.pop_start();
                newfile.export = public;

                parsed_file.imported.insert(name, newfile);
                continue;
            }
            Token::Function => {
                let name = tokens.parse_identifer();
                let function = function::parse_function(counter, &mut tokens, public);

                match parsed_file.functions.remove(&name) {
                    Some(old) => {
                        let message = tokens.throw(
                            MessageKind::Error,
                            old.location,
                            format!("There's already a function named '{}'", name),
                            "",
                        );
                        message.push("", function.location.clone());
                    }
                    None => {}
                }

                parsed_file.functions.insert(name.clone(), function);
                continue;
            }
            t => {
                tokens.throw(
                    MessageKind::Error,
                    info.location,
                    format!("Expected item, found '{}'", t),
                    "",
                );
                continue;
            }
        };
    }

    let mut file_messages = tokens.finish();
    file_messages.set_path(relative_path);
    messages.should_throw();
    parsed_file.messages = file_messages;
    // messages.push(file_messages);

    return parsed_file;
}
