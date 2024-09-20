use crate::{lexer::{Token, TokenInfo, TokensGroup}, parser::function::parse_function, CompileError, ParseResult};

use super::{identifier::parse_identifier, node::ASTNode, variable::parse_variable};

pub fn parse(tokens: &mut TokensGroup) -> ParseResult<Vec<ASTNode>> {
    let mut tree = Vec::new();

    loop {
        let info = tokens.start()?;

        let node: ASTNode = match info.token {
            Token::EndOfFile => break,
            Token::Function => parse_function(tokens, false, false)?,
            Token::Identifier(string) => parse_identifier(tokens, string)?,
            Token::Variable => parse_variable(tokens)?,
            token => return Err(CompileError::new(format!("Unhandled token: {:?}", token), tokens.current.line))
        };

        tree.push(node);
    }

    return Ok(tree);
} 

pub fn get_identifier(tokens: &mut TokensGroup) -> ParseResult<String> {
    let info = tokens.start()?;

    match info.token {
        Token::Identifier(name) => return Ok(name),
        _ => return Err(CompileError::new(String::from("Unhandled token"), tokens.current.line))
    }
}

pub fn expect_tokens(tokens: &mut TokensGroup, expected: Vec<Token>) -> ParseResult<TokenInfo> {
    let info = tokens.advance()?;
    let mut found = false;
    for token in &expected {
        if token == &info.token {
            found = true;
            break;
        }
    }
    
    if found {
        return Ok(info)
    }
    return Err(CompileError::new(format!("Expected tokens: {:?}", expected), tokens.current.line))
}
pub fn peek_expect_tokens(tokens: &mut TokensGroup, expected: Vec<Token>, advance_if_found: bool) -> ParseResult<TokenInfo> {
    let info = tokens.peek()?;
    let mut found = false;
    for token in &expected {
        if token == &info.token {
            found = true;
            break;
        }
    }
    
    if found {
        if advance_if_found {
            tokens.advance()?;
        }
        return Ok(info)
    }
    return Err(CompileError::new(format!("Expected tokens: {:?}", expected), tokens.current.line))
}