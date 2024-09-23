use crate::{
    lexer::{Token, TokensGroup},
    parser::parser::expect_tokens,
    ParseResult,
};

use super::{
    arguments::parse_arguments,
    node::{ASTNode, Node, Path},
};

pub fn parse_identifier(tokens: &mut TokensGroup, string: String) -> ParseResult<ASTNode> {
    let info = expect_tokens(
        tokens,
        vec![Token::Dot, Token::DoubleColon, Token::OpenParen],
    )?;
    let node: ASTNode = match info.token {
        Token::Dot => todo!(),
        Token::DoubleColon => todo!(),
        Token::OpenParen => {
            let arguments = parse_arguments(tokens)?;

            tokens.create_ast(Node::Call(Path::new(string), arguments))
        }
        _ => panic!(),
    };

    return Ok(node);
}
