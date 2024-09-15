use crate::{
    lexer::{Token, TokensGroup},
    BuildError, BuildProblem, CompileError,
};

use super::{
    arguments::parse_arguments,
    node::{Expression, Value},
    path::parse_path,
    tokens_expected_got, Operator,
};

pub fn parse_expression(tokens: &mut TokensGroup) -> Result<Option<Expression>, CompileError> {
    match tokens.peek() {
        Ok(info) => match info.token {
            Token::SemiColon => return Ok(None),
            _ => {}
        },
        Err(error) => return Err(error),
    }

    let minus = match tokens.peek() {
        Ok(info) => match info.token {
            Token::Minus => match tokens.advance() {
                Ok(_) => true,
                Err(error) => return Err(error),
            },
            _ => false,
        },
        Err(error) => return Err(error),
    };

    let info = match tokens.advance() {
        Ok(info) => info,
        Err(error) => return Err(error),
    };

    let expression = match info.token {
        Token::Integer(integer) => Expression::Value(match minus {
            true => Value::Integer(-(integer as isize)),
            false => Value::UInteger(integer),
        }),
        Token::Identifier(name) => {
            let path = match parse_path(tokens, name) {
                Ok(path) => path,
                Err(error) => return Err(error),
            };

            match tokens.peek() {
                Ok(info) => match info.token {
                    Token::OpenParen => {
                        tokens.advance().unwrap();

                        let arguments = match parse_arguments(tokens) {
                            Ok(args) => args,
                            Err(error) => return Err(error),
                        };

                        Expression::Call(path, arguments)
                    }
                    // Token::SemiColon => Expression::GetVariable(path),
                    _ => Expression::GetVariable(path),
                },
                Err(error) => return Err(error),
            }
        }
        _ => return Ok(None),
    };

    match tokens.peek() {
        Ok(info) => match info.token {
            Token::SemiColon | Token::Comma | Token::CloseParen => return Ok(Some(expression)),
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                tokens.advance().unwrap();

                let operator = match info.token {
                    Token::Plus => Operator::Plus,
                    Token::Minus => Operator::Minus,
                    Token::Asterisk => Operator::Multiply,
                    Token::Slash => Operator::Division,
                    _ => panic!(),
                };

                let second = match parse_expression(tokens) {
                    Ok(expression) => match expression {
                        Some(expression) => expression,
                        None => {
                            return Err(CompileError::BuildProblem(BuildProblem::new(
                                BuildError::ExpressionExpected,
                                tokens.relative_path.clone(),
                                tokens.current.lines.clone(),
                            )))
                        }
                    },
                    Err(error) => return Err(error),
                };
                return Ok(Some(Expression::BinaryOperation(
                    Box::new(expression),
                    operator,
                    Box::new(second),
                )));
            }
            _ => return Err(tokens_expected_got(tokens, vec![Token::SemiColon], info)),
        },
        Err(error) => return Err(error),
    }

    // return Ok(Some(expression));
}

// let expression = match info.token {
//     Token::Integer(integer) => Expression::Value(Value::Integer(integer as isize)),
//     Token::Identifier(name) => {
//         println!("{:?}", name);
//         let info = match tokens.peek() {
//             Ok(info) => info,
//             Err(error) => return Err(error),
//         };

//         match info.token {
//             Token::DoubleColon => {
//                 tokens.advance().unwrap();
//                 let path = match parse_path(tokens, name.clone()) {
//                     Ok(path) => path,
//                     Err(error) => return Err(error),
//                 };

//                 let info = match tokens.peek() {
//                     Ok(info) => info,
//                     Err(error) => return Err(error),
//                 };
//                 match info.token {
//                     Token::OpenParen => {
//                         tokens.advance().unwrap();
//                         let arguments = match parse_arguments(tokens) {
//                             Ok(arguments) => arguments,
//                             Err(error) => return Err(error),
//                         };

//                         return Ok(Some(Expression::Call(path, arguments)));
//                     }
//                     Token::CloseParen => return Ok(Some(Expression::LoadPath(path))),
//                     Token::Comma => return Ok(Some(Expression::LoadPath(path))),
//                     _ => todo!(),
//                 }
//             }
//             Token::OpenParen => {
//                 tokens.advance().unwrap();
//                 let arguments = match parse_arguments(tokens) {
//                     Ok(arguments) => arguments,
//                     Err(error) => return Err(error),
//                 };

//                 return Ok(Some(Expression::Call(Path::new(name), arguments)));
//             }
//             Token::CloseParen | Token::Comma | Token::SemiColon => return Ok(Some(Expression::GetVariable(name))),
//             _ => return Err(tokens_expected_got(tokens, vec![], info))
//         }
//     }
//     Token::CloseParen => return Ok(None),
//     _ => todo!(),
// };
