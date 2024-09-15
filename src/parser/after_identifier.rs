use crate::{
    lexer::{Token, TokensGroup},
    CompileError,
};

use super::{
    arguments::parse_arguments, expression::parse_expression, path::parse_path,
    tokens_expected_got, ASTNode, Node,
};

pub fn parse_after_identifier(
    tokens: &mut TokensGroup,
    name: String,
) -> Result<ASTNode, CompileError> {
    let path = match parse_path(tokens, name) {
        Ok(path) => path,
        Err(error) => return Err(error),
    };

    let info = match tokens.advance() {
        Ok(info) => info,
        Err(error) => return Err(error),
    };

    let node: ASTNode = match info.token {
        Token::Equals => {
            let expression = match parse_expression(tokens) {
                Ok(expression) => match expression {
                    Some(expression) => expression,
                    None => {
                        return Err(CompileError::BuildProblem)
                    }
                },
                Err(error) => return Err(error),
            };

            ASTNode::new(tokens.current.lines.clone(), Node::SetVariable(path, expression))
        }
        Token::OpenParen => {
            let arguments = match parse_arguments(tokens) {
                Ok(args) => args,
                Err(error) => return Err(error),
            };

            ASTNode::new(tokens.current.lines.clone(), Node::Call(path, arguments))
        }
        _ => {
            return Err(tokens_expected_got(
                tokens,
                vec![Token::Equals, Token::OpenParen],
                info,
            ))
        }
    };

    match tokens.advance() {
        Ok(info) => match info.token {
            Token::SemiColon => {}
            _ => return Err(tokens_expected_got(tokens, vec![Token::SemiColon], info)),
        },
        Err(error) => return Err(error),
    }

    return Ok(node);
}

pub fn parse_identifer_string(tokens: &mut TokensGroup) -> Result<String, CompileError> {
    return match tokens.advance() {
        Ok(info) => match info.token {
            Token::Identifier(str) => Ok(str),
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Identifier(String::from("enum"))],
                    info,
                ))
            }
        },
        Err(error) => return Err(error),
    };
}

// Token::Identifier(root) => {
//     let info = match tokens.advance() {
//         Ok(info) => info,
//         Err(error) => return Err(error),
//     };
//     match info.token {
//         Token::OpenParen => {
//             let arguments = match parse_arguments(tokens) {
//                 Ok(arguments) => arguments,
//                 Err(error) => return Err(error),
//             };
//             Ok(ASTNode::new(
//                 Node::Call(Path::new(root), arguments),
//                 tokens.current.line,
//             ))
//         }
//         Token::Equals => parse_set_variable(tokens, root),
//         Token::DoubleColon => {
//             let path = match parse_path(tokens, root) {
//                 Ok(path) => path,
//                 Err(error) => return Err(error),
//             };

//             match tokens.advance() {
//                 Ok(info) => match info.token {
//                     Token::OpenParen => {}
//                     _ => {
//                         return Err(tokens_expected_got(
//                             tokens,
//                             vec![Token::OpenParen],
//                             info,
//                         ))
//                     }
//                 },
//                 Err(error) => return Err(error),
//             }

//             let arguments = match parse_arguments(tokens) {
//                 Ok(arguments) => arguments,
//                 Err(error) => return Err(error),
//             };

//             Ok(ASTNode::new(
//                 Node::Call(path, arguments),
//                 tokens.current.line,
//             ))
//         }
//         _ => {
//             return Err(tokens_expected_got(
//                 tokens,
//                 vec![Token::OpenParen, Token::Equals, Token::DoubleColon],
//                 info,
//             ))
//         }
//     }
// }
