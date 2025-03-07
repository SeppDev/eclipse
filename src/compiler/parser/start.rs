use crate::{
    compiler::{lexer::token::Token, nodes::parser::ParserState},
    diagnostics::DiagnosticResult,
};

use super::Parser;

impl Parser {
    pub fn start_block(&mut self) -> DiagnosticResult<ParserState> {
        self.finish_statement()?;
        Ok(ParserState::Block(Vec::new()))
    }
    pub fn start_return(&mut self) -> DiagnosticResult<ParserState> {
        self.finish_statement()?;
        Ok(ParserState::Return(Vec::new()))
    }
    pub fn start_var_decl(&mut self) -> DiagnosticResult<ParserState> {
        self.finish_statement()?;
        let mutable = self.next_if_eq(Token::Mutable)?;

        let name = self.expect_identifier()?;
        let data_type = if self.next_if_eq(Token::Colon)?.is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(vec![Token::Equals])?;
        Ok(ParserState::VarDecl {
            mutable,
            name,
            data_type,
            value: Vec::new(),
        })
    }
    pub fn start_function(&mut self) -> DiagnosticResult<ParserState> {
        self.finish_statement()?;
        let name = self.expect_identifier()?;
        self.expect(vec![Token::OpenParen])?;
        let parameters = self.parse_parmeters()?;
        let return_type = match self.next_if_eq(Token::Colon)?.is_some() {
            true => Some(self.parse_type()?),
            false => None,
        };
        self.expect(vec![Token::OpenBlock])?;

        Ok(ParserState::Function {
            name,
            parameters,
            return_type,
            body: Vec::new(),
        })
    }
}
