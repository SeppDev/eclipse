use crate::{
    compiler::{lexer::token::Token, nodes::ast::Expression},
    diagnostics::DiagnosticResult,
};

use super::reader::TokenReader;

impl TokenReader {
    pub(super) fn parse_expression(&mut self) -> DiagnosticResult<Option<Expression>> {
        let mut expressions: Vec<Expression> = Vec::new();
        let current_expression = None;

        loop {
            let token = self.expect(&vec![Token::Function, Token::Variable])?;
            match token.raw {
                Token::Function => break,
                _ => todo!(),
            }
        }

        Ok(current_expression)
    }
}
