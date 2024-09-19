use crate::{lexer::TokensGroup, ParseResult};

use super::node::ASTNode;

pub fn parse(tokens: &mut TokensGroup) -> ParseResult<Vec<ASTNode>> {

    println!("{:#?}", tokens);
    todo!()
} 