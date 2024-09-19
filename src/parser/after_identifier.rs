use crate::{
    lexer::{Token, TokensGroup}, BuildError
};

use super::{
    arguments::parse_arguments, expression::parse_expression, path::parse_path,
    tokens_expected_got, ASTNode, Node,
};

pub fn parse_after_identifier(
    tokens: &mut TokensGroup,
    name: String,
) -> Result<ASTNode, BuildError> {
    let path = parse_path(tokens, name)?;
    let info = tokens.advance()?;

    let node: ASTNode = match info.token {
        Token::Equals => {
            let expression = parse_expression(tokens)?;
            let expression = match expression {
                Some(expression) => expression,
                None => {
                    return Err(tokens.create_error(format!("")))
                }
            };

            tokens.generate(Node::SetVariable(path, expression))?
        }
        Token::OpenParen => {
            let arguments = parse_arguments(tokens)?;

            tokens.generate(Node::Call(path, arguments))?
        }
        _ => {
            return Err(tokens_expected_got(
                tokens,
                vec![Token::Equals, Token::OpenParen],
                info,
            ))
        }
    };  
    


    let info = tokens.advance()?;
    match info.token {
        Token::SemiColon => {}
        _ => return Err(tokens_expected_got(tokens, vec![Token::SemiColon], info)),
    }

    return Ok(node);
}

pub fn parse_identifer_string(tokens: &mut TokensGroup) -> Result<String, BuildError> {
    let info = tokens.advance()?;
    match info.token {
        Token::Identifier(str) => Ok(str),
        _ => {
            return Err(tokens_expected_got(
                tokens,
                vec![Token::Identifier(String::from("enum"))],
                info,
            ))
        }
    }
}