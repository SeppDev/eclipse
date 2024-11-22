use crate::compiler::{
    counter::NameCounter,
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
    types::{BaseType, Type},
};

use super::{body::parse_body, types::parse_type};

pub fn parse_function(
    tokens: &mut Tokens,
    is_main: bool,
    name_counter: &mut NameCounter,
    export: bool,
) -> CompileResult<NodeInfo> {
    let name = tokens.parse_identifier()?;
    tokens.expect_tokens(vec![Token::OpenParen], false)?;

    let mut parameters: Vec<(String, Type)> = Vec::new();
    loop {
        if tokens
            .peek_expect_tokens(vec![Token::CloseParen], true)
            .is_some()
        {
            break;
        }
        let name = tokens.parse_identifier()?;
        let data_type = parse_type(tokens)?;
        parameters.push((name, data_type));

        let result = tokens.expect_tokens(vec![Token::CloseParen, Token::Comma], false)?;
        match result.token {
            Token::CloseParen => break,
            Token::Comma => continue,
            _ => break,
        }
    }

    let return_type = if tokens
        .peek_expect_tokens(vec![Token::Colon], true)
        .is_some()
    {
        parse_type(tokens)?
    } else {
        Type::Base(BaseType::Void)
    };

    tokens.expect_tokens(vec![Token::StartScope], false)?;
    let body = parse_body(tokens)?;

    return Ok(tokens.create_node(Node::Function {
        export,
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
