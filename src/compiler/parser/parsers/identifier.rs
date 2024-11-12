// pub fn parse_identifier() {

// }

use crate::compiler::{
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
    pub fn parse_identifer(&mut self) -> String {
        let info = self.advance();

        let token = match info.token {
            Token::Identifier(string) => return string,
            token => token.clone(),
        };

        self.throw_error(
            format!("Expected identifier, found '{}'", token),
            "expected identifier",
            &info.location
        )
    }
}

pub fn parse_after_identifier(tokens: &mut Tokens, name: String) -> NodeInfo {
    let info = tokens.peek_require_token(
        vec![Token::OpenParen, Token::Equals, Token::DoubleColon],
    );

    match info.token {
        Token::DoubleColon => {
            let path = parse_path(tokens, &name);
            let _ = tokens.expect_tokens(vec![Token::OpenParen], false);
            let arguments = parse_arguments(tokens);
            return tokens.create_node(Node::Call(path, arguments))
        },
        _ => {}
    }

    tokens.advance();
    return match info.token {
        Token::OpenParen => {
            let arguments = parse_arguments(tokens);
            tokens.create_node(Node::Call(Path::from(&name), arguments))
        }
        Token::Equals => parse_set_variable(tokens, name),
        _ => panic!(),
    };
}
