use crate::{
    common::position::Located,
    compiler::{
        lexer::token::{Token, TokenInfo},
        nodes::ast::{Expression, Parameter, RawParameter},
        parser::{ParsingState, StartState},
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

use super::Parser;

impl Parser {
    pub fn parse_expression(&mut self) -> DiagnosticResult<Expression> {
        loop {
            let token = self.expect(vec![
                Token::Function,
                Token::Variable,
                Token::Return,
                Token::StartBlock,
                Token::EndBlock,
            ])?;

            match token.raw {
                Token::Function => {
                    let name = self.expect_identifier()?;
                    self.expect(vec![Token::OpenParen])?;
                    let parameters = self.parse_parmeters()?;
                    let return_type = match self.next_if_eq(Token::Colon).is_some() {
                        true => Some(self.parse_type()?),
                        false => None,
                    };

                    let position = name.position;
                    self.states.push(StartState::new(
                        ParsingState::Function {
                            name,
                            parameters,
                            return_type,
                        },
                        position.start,
                    ));
                }
                Token::StartBlock => {}
                Token::EndBlock => {
                    self.handle_delimiter(token)?;
                    if self.states.len() == 0 {
                        break;
                    }
                }
                _ => todo!(),
            }
        }

        // Ok(expressions.pop())
        todo!()
    }
    pub fn handle_delimiter(&mut self, token: TokenInfo) -> DiagnosticResult<Expression> {
        let state = match self.states.pop() {
            Some(e) => e,
            None => {
                return Err(DiagnosticData::basic(
                    format!("Missing closing delimiter '{}'", token.raw),
                    self.path(),
                ))
            }
        };

        println!("{:#?}", self.states);

        match state.state {
            ParsingState::Function {
                name,
                parameters,
                return_type,
            } => {
                todo!()
            }
            // } => Ok(Expression::new(RawExpression::Function { name, parameters, return_type, body }, position))
            _ => todo!(),
        }
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
