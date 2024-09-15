use crate::{lexer::{Token, TokensGroup}, CompileError};

use super::{parse, tokens_expected_got, ASTNode};


pub fn parse_scope(tokens: &mut TokensGroup) -> Result<Vec<ASTNode>, CompileError> {
    
    let tree: Vec<ASTNode> = match parse(tokens) {
        Ok(tree) => tree,
        Err(error) => return Err(error)
    };

    let info = match tokens.advance() {
        Ok(info) => info,
        Err(error) => return Err(error)
    };

    match info.token {
        Token::EndScope => {},
        _ => return Err(tokens_expected_got(tokens, vec![Token::EndScope], info))
    }

    
    return Ok(tree); 
}