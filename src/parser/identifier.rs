use crate::{
    lexer::{Token, TokensGroup},
    parser::parser::expect_tokens,
    ParseResult,
};

use super::{
    arguments::parse_arguments,
    expression::parse_expected_expression,
    node::{ASTNode, Node, Path},
    path::parse_path,
};

pub fn parse_identifier(tokens: &mut TokensGroup, string: String) -> ParseResult<ASTNode> {
    let info = expect_tokens(
        tokens,
        vec![
            Token::Dot,
            Token::DoubleColon,
            Token::OpenParen,
            Token::Equals,
        ],
    )?;
    let node: ASTNode = match info.token {
        Token::Dot => todo!(),
        Token::Equals => {
            let expression = parse_expected_expression(tokens)?;
            tokens.create_ast(Node::SetVariable(string, expression))
        }
        Token::DoubleColon => {
            let path = parse_path(tokens, string)?;
            let _info = expect_tokens(tokens, vec![Token::OpenParen])?;
            let arguments = parse_arguments(tokens)?;

            tokens.create_ast(Node::Call(path, arguments))
        }
        Token::OpenParen => {
            let arguments = parse_arguments(tokens)?;

            tokens.create_ast(Node::Call(Path::new(string), arguments))
        }
        _ => panic!(),
    };

    return Ok(node);
}
