use crate::{
    lexer::{Token, TokensGroup},
    BuildError, BuildProblem, CompileError,
};

use super::{
    expression::parse_expression,
    node::{ASTNode, Node},
    tokens_expected_got,
    types::parse_type,
    Expression,
};

pub fn parse_define_variable(tokens: &mut TokensGroup) -> Result<ASTNode, CompileError> {
    let mutable = match tokens.peek() {
        Ok(info) => match info.token {
            Token::Mutable => {
                tokens.advance().unwrap();
                true
            },
            _ => false,
        },
        Err(error) => return Err(error),
    };

    let name = match tokens.advance() {
        Ok(info) => match info.token {
            Token::Identifier(name) => name,
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Identifier(String::from("variable"))],
                    info,
                ))
            }
        },
        Err(error) => return Err(error),
    };

    let var_type = match tokens.peek() {
        Ok(info) => match info.token {
            Token::Equals => None,
            Token::Colon => {
                tokens.advance().unwrap();
                match parse_type(tokens) {
                    Ok(t) => Some(t),
                    Err(error) => return Err(error),
                }
            }
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Colon, Token::Equals],
                    info,
                ))
            }
        },
        Err(error) => return Err(error),
    };

    let expression: Option<Expression> = match tokens.advance() {
        Ok(info) => match info.token {
            Token::Equals => match parse_expression(tokens) {
                Ok(expression) => match expression {
                    Some(expression) => Some(expression),
                    None => {
                        return Err(CompileError::BuildProblem(BuildProblem::new(
                            BuildError::ExpressionExpected,
                            tokens.relative_path.clone(),
                            tokens.current.line,
                        )))
                    }
                },
                Err(error) => return Err(error),
            },
            _ => None,
        },
        Err(error) => return Err(error),
    };

    match tokens.advance() {
        Ok(info) => match info.token {
            Token::SemiColon => {}
            _ => return Err(tokens_expected_got(tokens, vec![Token::SemiColon], info)),
        },
        Err(error) => return Err(error),
    }

    return Ok(ASTNode::new(
        tokens.current.line,
        Node::DefineVariable {
            mutable,
            name,
            var_type,
            expression,
        },
    ));
}

// pub fn parse_set_variable(tokens: &mut TokensGroup, name: String) -> Result<ASTNode, CompileError> {
//     let expression = match parse_expression(tokens) {
//         Ok(expression) => match expression {
//             Some(expression) => expression,
//             None => {
//                 return Err(CompileError::BuildProblem(BuildProblem::new(
//                     BuildError::ExpressionExpected,
//                     tokens.relative_path.clone(),
//                     tokens.current.line,
//                 )))
//             }
//         },
//         Err(error) => return Err(error),
//     };

//     return Ok(ASTNode::new(
//         Node::SetVariable(Path::new(name), expression),
//         tokens.current.line,
//     ));
// }
