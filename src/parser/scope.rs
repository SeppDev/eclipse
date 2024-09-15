use crate::{
    lexer::{Token, TokensGroup},
    CompileError,
};

use super::{parse, tokens_expected_got, ASTNode};

pub fn parse_scope(tokens: &mut TokensGroup) -> Result<Vec<ASTNode>, CompileError> {
    let tree: Vec<ASTNode> = parse(tokens)?;
    let info = tokens.advance()?;
    match info.token {
        Token::EndScope => {}
        _ => return Err(tokens_expected_got(tokens, vec![Token::EndScope], info)),
    }

    return Ok(tree);
}
