use std::path::PathBuf;

use eclipse::{read_file, CompileError, ParseError};
use tokens::parse_tokens;

use super::lexer::{tokenize, Token, TokenInfo};
use super::node::Node;
use super::tokens_group::TokensGroup;

mod tokens;
mod scope;
mod function;
mod parse_type;
mod variable;
mod result;
mod call;
mod conditional;
mod module;

pub fn parse(path: &PathBuf) -> Result<Vec<Node>, CompileError> {
    let path_str = match path.to_str() {
        Some(s) => s,
        None => panic!("Path not found!?"),
    };

    let source = match read_file(path_str) {
        Ok(source) => source,
        Err(error) => return Err(error),
    };
    
    let tokens = match tokenize(source) {
        Ok(tokens) => tokens,
        Err(error) => return Err(error),
    };

    let nodes = match parse_tokens(&mut TokensGroup::new(&mut tokens.iter().peekable())) {
        Ok(nodes) => nodes,
        Err(error) => return Err(CompileError::Parsing(error)),
    };


    return Ok(nodes);
}

pub fn token_expected(expected: Token, got: TokenInfo) -> ParseError {
    return ParseError::TokenExpected(format!("Expected {:?} got: {:?}", expected, got));
}