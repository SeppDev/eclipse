use std::path::PathBuf;

mod expect_token;
mod function;
mod identifier;
mod types;
mod body;
mod expression;
mod variable;
mod arguments;

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

        let node = match info.token.clone() {
            Token::EndOfFile => break,
            Token::Import => {
                let _name = tokens.parse_identifer();
                // parse(project_dir, relative_path.parent().unwrap().join(name));
                todo!()
            }
            Token::Function => function::parse_function(&mut tokens),
            t => tokens.throw_error(format!("Expected item, found '{}'", t), ""),
        };
        
        println!("{:#?}", node);
    }
}
