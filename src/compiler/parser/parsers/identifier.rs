use crate::compiler::{
    errors::CompileResult,
    parser::{Node, NodeInfo},
    path::Path,
};

use super::{
    super::super::lexer::{Token, Tokens},
    arguments::parse_arguments,
    path::parse_path,
    variable::parse_set_variable,
};
impl Tokens {
    pub fn parse_identifier(&mut self) -> CompileResult<String> {
        let current = self.current().clone();
        let info = self.advance();

        if let Token::Identifier(name) = info.token {
            return Ok(name);
        } else {
            self.error(
                current.location,
                format!("Expected identifier, found '{}'", info.token),
            );
        }

        return Ok("x".to_string());
    }
}

pub fn parse_after_identifier(tokens: &mut Tokens, name: String) -> CompileResult<NodeInfo> {
    let info =
        tokens.peek_require_tokens(vec![Token::OpenParen, Token::Equals, Token::DoubleColon]);

    match info.token {
        Token::DoubleColon => {
            let path = parse_path(tokens, &name)?;
            let _ = tokens.expect_tokens(vec![Token::OpenParen], false);
            let arguments = parse_arguments(tokens)?;
            return Ok(tokens.create_node(Node::Call(path, arguments)));
        }
        _ => {}
    }

    tokens.advance();
    return match info.token {
        Token::OpenParen => {
            let arguments = parse_arguments(tokens)?;
            Ok(tokens.create_node(Node::Call(Path::from(&name), arguments)))
        }
        Token::Equals => parse_set_variable(tokens, name),
        _ => panic!(),
    };
}
