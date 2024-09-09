use crate::{
    lexer::{Token, TokensGroup},
    CompileError,
};

use super::{
    after_identifier::parse_identifer_string, enums::parse_enum, function::parse_function,
    structs::parse_struct, tokens_expected_got, ASTNode, Node,
};

pub fn parse_export(tokens: &mut TokensGroup) -> Result<ASTNode, CompileError> {
    match tokens.peek() {
        Ok(info) => match info.token {
            Token::Enum => {
                tokens.advance().unwrap();
                return parse_enum(tokens, true);
            }
            Token::Struct => {
                tokens.advance().unwrap();
                return parse_struct(tokens, true);
            }
            Token::Import => {
                tokens.advance().unwrap();
                let name = match parse_identifer_string(tokens) {
                    Ok(str) => str,
                    Err(error) => return Err(error),
                };
                return Ok(ASTNode::new(tokens.current.line, Node::Import(name, true)));
            }
            _ => {}
        },
        Err(error) => return Err(error),
    };

    let is_unsafe = match tokens.peek() {
        Ok(info) => match info.token {
            Token::Unsafe => {
                tokens.advance().unwrap();
                true
            }
            _ => false,
        },
        Err(error) => return Err(error),
    };
    match tokens.advance() {
        Ok(info) => match info.token {
            Token::Function => parse_function(tokens, true, is_unsafe),
            _ => return Err(tokens_expected_got(tokens, vec![Token::Function], info)),
        },
        Err(error) => return Err(error),
    }
}
