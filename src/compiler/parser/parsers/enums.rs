use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

pub fn parse_enum(tokens: &mut Tokens) -> CompileResult<NodeInfo> {
    let name = tokens.parse_identifier()?;
    let _ = tokens.expect_tokens(vec![Token::StartScope], false);
    let mut fields = Vec::new();

    loop {
        if tokens
            .peek_expect_tokens(vec![Token::EndScope], true)
            .is_some()
        {
            break;
        };

        let name = tokens.parse_identifier()?;
        fields.push(name);

        let result = tokens.expect_tokens(vec![Token::Comma, Token::EndScope], false)?;
        match result.token {
            Token::Comma => continue,
            Token::EndScope => break,
            _ => panic!(),
        }
    }

    return Ok(tokens.create_node(Node::Enum { name, fields }));
}
