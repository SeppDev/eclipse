use eclipse::ParseError;

use crate::parser::{
    lexer::Token,
    node::{Expression, Node, Type},
    tokens_group::TokensGroup,
};

use super::{parse_type::parse_type, result::result};

pub fn variable(tokens: &mut TokensGroup) -> Result<Node, ParseError> {
    let mutable = match tokens.peek().unwrap().token {
        Token::Mutable => {
            tokens.next_token();
            true
        }
        _ => false,
    };
    let name = match tokens.next_token().unwrap().token {
        Token::Identifier(name) => name,
        _ => panic!(),
    };
    let var_type = match tokens.peek().unwrap().token {
        Token::Colon => {
            tokens.next_token();
            Some(parse_type(tokens))
        }
        _ => None,
    };
    let expression: Option<Expression> = match tokens.peek().unwrap().token {
        Token::Equals => {
            tokens.next_token();
            Some(match result(tokens) {
                Ok(a) => match a {
                    Some(a) => a,
                    None => return Err(ParseError::NoTokenFound)
                },
                Err(error) => return Err(error),
            })
        }
        _ => None,
    };

    let var_type: Option<Type> = match var_type {
        Some(t) => match t {
            Ok(a) => Some(a),
            Err(error) => return Err(error),
        },
        None => None,
    };

    Ok(Node::DefineVariable {
        name: name.to_owned(),
        mutable: mutable,
        var_type: var_type,
        expression: expression,
    })
}
