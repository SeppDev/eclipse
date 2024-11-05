use std::path::PathBuf;

mod arguments;
mod body;
mod expect_token;
mod expression;
mod function;
mod identifier;
mod types;
mod variable;

use crate::compiler::{lexer::tokenize, read_file};

fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

pub fn parse(project_dir: &PathBuf, relative_path: PathBuf) {
    use super::super::lexer::Token;

    let file_path = project_dir.join(&relative_path);
    let source = read_file(&file_path);

    let lexer_start = std::time::Instant::now();
    let mut tokens = tokenize(&relative_path, source);
    let lexer_took = lexer_start.elapsed();

    let mut nodes = Vec::new();
    let parser_start = std::time::Instant::now();

    loop {
        if tokens.is_eof() {
            break;
        }

        let info = tokens.expect_tokens(vec![Token::Import, Token::Function], true);

        let node = match info.token {
            Token::Import => {
                let _name = tokens.parse_identifer();
                // parse(project_dir, relative_path.parent().unwrap().join(name));
                tokens.pop_start();
                todo!()
            }
            Token::Function => function::parse_function(&mut tokens),
            t => tokens.throw_error(format!("Expected item, found '{}'", t), ""),
        };
        nodes.push(node);
    }
    tokens.finish();
    let parser_took = parser_start.elapsed();
    // println!("{:#?}", nodes);

    println!("lexer: {:?}\nparser: {:?}", lexer_took, parser_took);
}
