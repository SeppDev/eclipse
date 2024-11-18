use std::{collections::HashMap, path::PathBuf};

mod arguments;
mod body;
mod expect_token;
mod expression;
mod function;
mod identifier;
mod ifstatement;
mod import;
mod namespace;
mod path;
mod types;
mod variable;

use function::parse_function;
use import::handle_import;

use crate::compiler::{
    errors::{CompileMessages, CompileResult},
    lexer::tokenize,
    path::Path,
    read_file, FILE_EXTENSION,
};

use super::NodeInfo;

#[derive(Debug)]
pub struct ParsedFile {
    pub imports: Vec<(String, ParsedFile)>,
    pub body: Vec<NodeInfo>,
    pub relative_path: Path,
}

pub fn start_parse(
    compile_messages: &mut CompileMessages,
    project_dir: &PathBuf,
    relative_path: Path,
) -> CompileResult<ParsedFile> {
    let mut file_path = {
        // let first = path.first().unwrap();
        project_dir.join(relative_path.convert())
    };
    file_path.set_extension(FILE_EXTENSION);

    let source = read_file(&file_path);

    let start = std::time::Instant::now();
    let mut tokens = tokenize(compile_messages, relative_path.clone(), source);
    println!("{:?}", start.elapsed());

    let mut imports = Vec::new();
    let mut body = Vec::new();

    use super::super::lexer::Token;
    loop {
        if tokens.is_eof() {
            break;
        }

        let info = tokens.expect_tokens(vec![Token::Import, Token::Function, Token::Use], true)?;

        match info.token {
            Token::Import => {
                let (name, import) =
                    handle_import(compile_messages, project_dir, &relative_path, &mut tokens)?;
                imports.push((name, import));
            }
            Token::Function => {
                let function = match parse_function(&mut tokens, false) {
                    Ok(f) => f,
                    Err(()) => break,
                };
                body.push(function)
            },
            Token::Enum => todo!(),
            Token::Struct => todo!(),
            _ => continue,
        }
    }

    tokens.finish(compile_messages);

    let file = ParsedFile {
        imports,
        body,
        relative_path,
    };

    return Ok(file);
}

// fn parse_tokens(
//     messages: &mut CompileMessages,
//     project_dir: &PathBuf,
//     relative_path: Path,
// ) -> ParsedFile {

//     loop {
//         if tokens.is_eof() {
//             break;
//         }

//         let public = tokens.peek_expect_tokens(vec![Token::Pub], true).is_some();
//         let info = tokens.expect_tokens(vec![Token::Import, Token::Function, Token::Use], true);

//         match info.token {
//             Token::Use => parse_namespace(&mut tokens, public),
//             Token::Import => {
//                 let name = tokens.parse_identifer();
//                 let relative_path = relative_path.parent().join(&name);

//                 let mut file_path = project_dir.join(relative_path.convert());
//                 file_path.set_extension(FILE_EXTENSION);

//                 let source = read_file(&file_path);
//                 let newfile = parse(counter, messages, project_dir, relative_path, source);
//                 tokens.pop_start();

//                 parsed_file.imported.insert(name, newfile);
//                 continue;
//             }
//             Token::Function => {
//                 let name = tokens.parse_identifer();
//                 let function = function::parse_function(counter, &mut tokens, public);

//                 match parsed_file.functions.remove(&name) {
//                     Some(old) => {
//                         let message = tokens.throw(
//                             MessageKind::Error,
//                             old.location,
//                             format!("There's already a function named '{}'", name),
//                             "",
//                         );
//                         message.push("", function.location.clone());
//                     }
//                     None => {}
//                 }

//                 parsed_file.functions.insert(name.clone(), function);
//                 continue;
//             }
//             t => {
//                 tokens.throw(
//                     MessageKind::Error,
//                     info.location,
//                     format!("Expected item, found '{}'", t),
//                     "",
//                 );
//                 continue;
//             }
//         };
//     }

//     let mut file_messages = tokens.finish();
//     file_messages.set_path(relative_path);
//     parsed_file.messages = file_messages;

//     return parsed_file;
// }
