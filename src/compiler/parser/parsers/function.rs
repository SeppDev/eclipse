use crate::compiler::{
    counter::NameCounter,
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo, Parameter},
    types::Type,
};

use super::{body::parse_body, types::parse_type};

pub fn parse_function(
    tokens: &mut Tokens,
    is_main: bool,
    name_counter: &mut NameCounter,
    _export: bool,
) -> CompileResult<NodeInfo> {
    let name = tokens.parse_identifier()?;
    tokens.expect_tokens(vec![Token::OpenParen], false)?;

    let mut parameters: Vec<Parameter> = Vec::new();
    if tokens
        .peek_expect_tokens(vec![Token::CloseParen], true)
        .is_none()
    {
        loop {
            tokens.start_next();
    
            let mutable = tokens
                .peek_expect_tokens(vec![Token::Mutable], true)
                .is_some();
    
            let name = tokens.parse_identifier()?;
            let data_type = parse_type(tokens)?;
            
            let mut location = tokens.pop_start().location;
            let current = tokens.current();
            location.lines.end = current.location.lines.end;
            location.columns.end = current.location.columns.end;

            let parameter = Parameter {
                location,
                mutable,
                name,
                data_type,
            };

            parameters.push(parameter);
    
            let result = &tokens.expect_tokens(vec![Token::CloseParen, Token::Comma], false)?;
            match result.token {
                Token::CloseParen => break,
                Token::Comma => continue,
                _ => break,
            }
        }
    }

    let return_type = if tokens
        .peek_expect_tokens(vec![Token::Colon], true)
        .is_some()
    {
        parse_type(tokens)?
    } else {
        Type::void()
    };

    
    tokens.expect_tokens(vec![Token::StartScope], false)?;
    let body = parse_body(tokens)?;

    return Ok(tokens.create_node(Node::Function {
        key: if is_main && name == "main" {
            "main".to_string()
        } else {
            name_counter.increment()
        },
        name,
        parameters,
        return_type,
        body,
    }));
}
