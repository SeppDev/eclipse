use crate::{
    lexer::{Token, TokensGroup}, BuildError
};

use super::{
    expression::parse_expression,
    node::{ASTNode, Node},
    tokens_expected_got,
    types::parse_type,
    Expression,
};

pub fn parse_define_variable(tokens: &mut TokensGroup) -> Result<ASTNode, BuildError> {
    let info = tokens.peek()?;
    let mutable = match info.token {
        Token::Mutable => {
            tokens.advance()?;
            true
        }
        _ => false,
    };

    let info = tokens.advance()?;
    let name = match info.token {
        Token::Identifier(name) => name,
        _ => {
            return Err(tokens_expected_got(
                tokens,
                vec![Token::Identifier(String::from("variable"))],
                info,
            ))
        }
    };

    let info = tokens.peek()?;
    let data_type = match info.token {
        Token::Equals => None,
        Token::Colon => {
            tokens.advance()?;
            Some(parse_type(tokens)?)
        }
        _ => {
            return Err(tokens_expected_got(
                tokens,
                vec![Token::Colon, Token::Equals],
                info,
            ))
        }
    };

    let info = tokens.advance()?;
    let expression: Option<Expression> = match info.token {
        Token::Equals => parse_expression(tokens)?,
        _ => None,
    };

    let info = tokens.advance()?;
    match info.token {
        Token::SemiColon => {}
        _ => return Err(tokens_expected_got(tokens, vec![Token::SemiColon], info)),
    }

    return Ok(ASTNode::new(lines, Node::DefineVariable {
        mutable,
        name,
        data_type,
        expression,
    }));
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
