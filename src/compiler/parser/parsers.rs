use std::path::PathBuf;
mod identifier;

use crate::compiler::{lexer::tokenize, read_file};

fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

pub fn parse(project_dir: &PathBuf, relative_path: PathBuf) {
    use super::super::lexer::Token;

    let file_path = project_dir.join(&relative_path);
    let source = read_file(&file_path);
    let mut tokens = tokenize(&relative_path, source);

    loop {
        let info = tokens.start();

        match info.token.clone() {
            Token::EndOfFile => break,
            Token::Import => {
                let name = tokens.parse_identifer();
                // parse(project_dir, relative_path.parent().unwrap().join(name));


            }
            Token::Function => {
                let name = tokens.parse_identifer();

            }
            t => tokens.create_error(format!("Expected item, found '{}'", t), ""),
        };
    }
}
