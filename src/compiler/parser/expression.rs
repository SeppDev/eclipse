use crate::{
    compiler::{lexer::token::Token, nodes::ast::Expression},
    diagnostics::DiagnosticResult,
};

use super::reader::TokenReader;

impl TokenReader {
    pub(super) fn parse_expression(&mut self) -> DiagnosticResult<Expression> {
        let token = self.expect(&vec![Token::Function, Token::Variable])?;

        todo!("{token:#?}")
    }
}
