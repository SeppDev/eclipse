use crate::{
    common::position::Located,
    compiler::{
        lexer::token::Token,
        nodes::{ast::RawParameter, parser::ParserState},
    },
    diagnostics::DiagnosticResult,
};

use super::Parser;

impl Parser {
    pub fn start_var_decl(&mut self) -> DiagnosticResult<ParserState> {
        let mutable = self.next_if_eq(Token::Mutable);

        let name = self.expect_identifier()?;
        let data_type = if self.next_if_eq(Token::Colon).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(vec![Token::Equals])?;
        Ok(ParserState::VarDecl {
            mutable,
            name,
            data_type,
        })
    }
    pub fn start_function(&mut self) -> DiagnosticResult<ParserState> {
        let name = self.expect_identifier()?;
        self.expect(vec![Token::OpenParen])?;
        let parameters = self.parse_parmeters()?;
        let return_type = match self.next_if_eq(Token::Colon).is_some() {
            true => Some(self.parse_type()?),
            false => None,
        };
        self.expect(vec![Token::OpenBlock])?;

        Ok(ParserState::Function {
            name,
            parameters,
            return_type,
        })
    }
    pub fn parse_parmeters(&mut self) -> DiagnosticResult<Vec<Parameter>> {
        let mut parameters = Vec::new();

        loop {
            if self.next_if_eq(Token::CloseParen).is_some() {
                break;
            }

            let reference = self.next_if_eq(Token::Ampersand);
            let mutable = self.next_if_eq(Token::Mutable);
            let name = self.expect_identifier()?;
            let data_type = self.parse_type()?;

            let position = name.position;

            let parameter = RawParameter {
                reference,
                mutable,
                name,
                data_type,
            };

            // TODO: accurate parameter position
            parameters.push(Located::new(parameter, position));
        }

        Ok(parameters)
    }
}
