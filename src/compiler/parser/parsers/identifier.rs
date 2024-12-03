use crate::compiler::{
    errors::CompileResult,
    parser::{ArithmeticOperator, Expression, ExpressionInfo, Node, NodeInfo},
    path::Path,
    types::ReferenceState,
};

use super::{
    super::super::lexer::{Token, Tokens}, arguments::parse_arguments, expression::parse_expression, path::parse_path, variable::parse_set_variable
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
    let info = tokens.peek_require_tokens(vec![
        Token::OpenParen,
        Token::Equals,
        Token::PlusEquals,
        Token::SubtractEquals,
        Token::DivideEquals,
        Token::MultiplyEquals,
        Token::DoubleColon,
    ]);

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
    return Ok(match info.token {
        Token::OpenParen => {
            let arguments = parse_arguments(tokens)?;
            tokens.create_node(Node::Call(Path::from(&name), arguments))
        }
        Token::Equals => parse_set_variable(tokens, name)?,
        Token::PlusEquals | Token::SubtractEquals | Token::MultiplyEquals | Token::DivideEquals => {
            let variable = ExpressionInfo {
                location: info.location.clone(),
                ref_state: ReferenceState::None,
                expression: Expression::GetVariable(Path::from(&name)),
            };
            let expression = parse_expression(tokens, true)?.unwrap();

            let operator = match info.token {
                Token::PlusEquals  => ArithmeticOperator::Plus,
                Token::SubtractEquals=> ArithmeticOperator::Subtract,
                Token::MultiplyEquals=> ArithmeticOperator::Multiply,
                Token::DivideEquals => ArithmeticOperator::Division,
                _ => panic!()
            };

            let binary_expression = Expression::BinaryOperation(
                Box::new(variable),
                operator,
                Box::new(expression),
            );
            
            let binary_expression = ExpressionInfo {
                location: info.location.clone(),
                ref_state: ReferenceState::None,
                expression: binary_expression,
            };

            return Ok(tokens.create_node(Node::SetVariable { name, expression: Some(binary_expression) }));
        }
        _ => panic!(),
    });
}
