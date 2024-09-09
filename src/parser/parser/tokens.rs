use eclipse::ParseError;

use super::{
    call::call_function, conditional::conditional, function::function, module::module, result::result, scope::scope, token_expected, variable::variable, Node
};
use crate::parser::{lexer::Token, tokens_group::TokensGroup};

pub fn parse_tokens(group: &mut TokensGroup) -> Result<Vec<Node>, ParseError> {
    let mut tree: Vec<Node> = Vec::new();

    loop {
        let info = group.peek().unwrap();
        let info = match info.token {
            Token::StartScope => {
                group.next_token();
                let nodes = match scope(group) {
                    Ok(nodes) => nodes,
                    Err(error) => return Err(error),
                };
                tree.push(Node::Scope(nodes));
                continue;
            }
            Token::EndScope => break,
            _ => group.next_token().unwrap(),
        };

        let node = match info.token {
            Token::EndOfFile => break,
            Token::Module => module(group),
            Token::Pub => match group.next_token() {
                Some(info) => match info.token {
                    Token::Function => function(group, true),
                    _ => return Err(token_expected(Token::Function, info))
                }
                None => return Err(ParseError::NoTokenFound),
            }
            Token::Function => function(group, false),
            Token::Variable => variable(group),
            Token::If => conditional(group),
            Token::Identifier(name) => match group.next_token() {
                Some(tokeninfo) => match tokeninfo.token {
                    Token::OpenParen => Ok(Node::Call(
                        name,
                        match call_function(group) {
                            Ok(a) => a,
                            Err(error) => return Err(error),
                        },
                    )),
                    Token::Equals => Ok(Node::SetVariable {
                        name: name,
                        expression: match result(group) {
                            Ok(a) => match a {
                                Some(a) => a,
                                None => return Err(ParseError::NoTokenFound),
                            },
                            Err(error) => return Err(error),
                        },
                    }),
                    _ => return Err(token_expected(Token::Equals, tokeninfo)),
                },
                None => return Err(ParseError::NoTokenFound),
            },
            Token::Return => Ok(Node::Return(match result(group) {
                Ok(a) => a,
                Err(error) => return Err(error),
            })),
            // token => todo!("{:#?}\n{:?}", group, token),
            token => panic!("{:?}", token)
        };

        match node {
            Ok(node) => tree.push(node),
            Err(error) => return Err(error),
        }
    }

    return Ok(tree);
}
