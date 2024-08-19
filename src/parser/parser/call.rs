use eclipse::ParseError;

use crate::parser::{lexer::Token, node::Expression, tokens_group::TokensGroup};

use super::{result::result, token_expected};

pub fn call_function(tokens: &mut TokensGroup) -> Result<Vec<Expression>, ParseError> {
    let mut arguments: Vec<Expression> = Vec::new();
    loop {
        match tokens.peek() {
            Some(tokeninfo) => match tokeninfo.token {
                Token::CloseParen => {
                    tokens.next_token().unwrap();
                    break;
                }
                _ => {
                    arguments.push(match result(tokens) {
                        Ok(a) => match a {
                            Some(a) => a,
                            None => return Err(ParseError::NoTokenFound)
                        },
                        Err(error) => return Err(error),
                    });
                    match tokens.next_token().unwrap().token {
                        Token::Comma => {}
                        Token::CloseParen => break,
                        _ => return Err(token_expected(Token::Comma, tokeninfo)),
                    }
                }
            },
            None => return Err(ParseError::NoTokenFound),
        }
    }

    Ok(arguments)
}
