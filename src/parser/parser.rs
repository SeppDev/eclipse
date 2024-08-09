use crate::lexer::{self, token::Token};

use super::{
    node::{Expression, Node, Type},
    tokens::Tokens,
};

fn parse_type(tokens: &mut Tokens) -> Type {
    let string = match tokens.next_token().unwrap() {
        Token::Identifier(string) => string,
        _ => panic!(),
    };
    return match string.as_str() {
        "i64" => Type::i64,
        _ => todo!(),
    };
}

fn result(tokens: &mut Tokens) -> Expression {
    return match tokens.next_token().unwrap() {
        Token::Integer(integer) => Expression::Value(integer),
        Token::Identifier(name) => {
            let expression = Expression::GetVariable(name.clone());
            // let is_reference = match tokens.peek().unwrap() {
            //     Token::Reference => true,
            //     _ => false,
            // };

            expression
        }
        token => panic!("{:?}", token),
    };
}

fn function(tokens: &mut Tokens) -> Node {
    let name = match tokens.next_token().unwrap() {
        Token::Identifier(name) => name,
        _ => panic!(),
    };

    match tokens.next_token().unwrap() {
        Token::OpenParen => {}
        _ => panic!(),
    }

    let mut parameters = Vec::new();
    loop {
        match tokens.next_token().unwrap() {
            Token::CloseParen => break,
            Token::Identifier(name) => {
                match tokens.next_token().unwrap() {
                    Token::Colon => {}
                    token => panic!("Expected ':' got: {:?}", token),
                }
                parameters.push((name, parse_type(tokens)));
            }
            _ => todo!(),
        }
        match tokens.peek().unwrap() {
            Token::Comma => {
                tokens.next_token();
            }
            Token::CloseParen => {
                tokens.next_token();
                break;
            }
            token => panic!("Expected ',' got: {:?}", token),
        }
    }

    Node::Function {
        name: name,
        parameters,
        return_types: None,
        body: match tokens.next_token().unwrap() {
            Token::StartScope => scope(tokens),
            _ => panic!(),
        },
    }
}

fn variable(tokens: &mut Tokens) -> Node {
    let mutable = match tokens.peek().unwrap() {
        Token::Mutable => {
            tokens.next_token();
            true
        }
        _ => false,
    };
    let name = match tokens.next_token().unwrap() {
        Token::Identifier(name) => name,
        _ => panic!(),
    };
    let var_type = match tokens.peek().unwrap() {
        Token::Colon => {
            tokens.next_token();
            Some(parse_type(tokens))
        }
        _ => None,
    };
    let expression: Option<Expression> = match tokens.peek().unwrap() {
        Token::Equals => {
            tokens.next_token();
            Some(result(tokens))
        }
        _ => None,
    };

    let var_type = match var_type {
        Some(t) => t,
        None => todo!(),
    };

    Node::DefineVariable {
        name: name,
        mutable: mutable,
        var_type: var_type,
        expression: expression,
    }
}

fn call_function(tokens: &mut Tokens) -> Vec<Expression> {
    let mut arguments = Vec::new();
    loop {
        match tokens.peek().unwrap() {
            Token::CloseParen => {
                tokens.next_token().unwrap();
                break;
            }
            _ => {
                arguments.push(result(tokens));
                match tokens.next_token().unwrap() {
                    Token::Comma => {}
                    Token::CloseParen => break,
                    _ => panic!(),
                }
            }
        }
    }

    arguments
}

fn conditional(tokens: &mut Tokens) -> Node {
    match tokens.next_token().unwrap() {
        Token::OpenParen => {}
        token => panic!("Expected '(' got, {:?}", token),
    }

    let a = result(tokens);
    match tokens.next_token().unwrap() {
        Token::Compare => {}
        token => panic!("Expected '==' got, {:?}", token),
    }
    let b = result(tokens);
    match tokens.next_token().unwrap() {
        Token::CloseParen => {}
        token => panic!("Expected ')' got, {:?}", token),
    }

    match tokens.next_token().unwrap() {
        Token::StartScope => {}
        _ => panic!(),
    }

    let body = scope(tokens);
    let else_body: Option<Vec<Node>> = match tokens.peek().unwrap() {
        Token::Else => {
            tokens.next_token().unwrap();
            match tokens.next_token().unwrap() {
                Token::StartScope => Some(scope(tokens)),
                _ => panic!(),
            }
        }
        _ => None,
    };

    Node::Conditional((a, b), body, else_body)
}

fn scope(tokens: &mut Tokens) -> Vec<Node> {
    let nodes = parse_tokens(tokens);
    match tokens.next_token().unwrap() {
        Token::EndScope => {}
        token => panic!("Expected '{}' got {:?}", '}', token),
    }
    return nodes;
}

fn parse_tokens(tokens: &mut Tokens) -> Vec<Node> {
    let mut tree: Vec<Node> = Vec::new();

    loop {
        let token = match tokens.peek().unwrap() {
            Token::StartScope => {
                tokens.next_token();
                tree.push(Node::Scope(scope(tokens)));
                continue;
            }
            Token::EndScope => break,
            _ => tokens.next_token().unwrap(),
        };
        let node = match token {
            Token::EndOfFile => break,
            Token::Function => function(tokens),
            Token::Variable => variable(tokens),
            Token::If => conditional(tokens),
            Token::Identifier(name) => {
                tokens.next_token();
                Node::Call(name, call_function(tokens))
            }
            token => todo!("{:?}", token),
        };
        tree.push(node);
    }
    
    return tree;
}

pub fn parse(source: String) -> Result<Vec<Node>, String> {
    let tokens = match lexer::lexer::tokenize(source) {
        Ok(tokens) => tokens,
        Err(error) => return Err(error),
    };

    return Ok(parse_tokens(&mut Tokens::new(
        &mut tokens.iter().peekable(),
    )));
}
