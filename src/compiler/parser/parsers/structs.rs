use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

use super::types::parse_type;

pub fn parse_struct(tokens: &mut Tokens) -> CompileResult<NodeInfo> {
    let name = tokens.parse_identifier()?;
    let _ = tokens.expect_tokens(vec![Token::StartScope], false);
    let mut fields = Vec::new();

    if tokens
        .peek_expect_tokens(vec![Token::EndScope], true)
        .is_some()
    {
        return Ok(tokens.create_node(Node::Struct { name, fields }))   
    };

    loop {
        let name = tokens.parse_identifier()?;
        let data_type = parse_type(tokens)?;
        fields.push((name, data_type));

        let result = tokens.expect_tokens(vec![Token::Comma, Token::EndScope], false)?;
        match result.token {
            Token::Comma => continue,
            Token::EndScope => break,
            _ => panic!(),
        }
    }

    return Ok(tokens.create_node(Node::Struct { name, fields }));
}
