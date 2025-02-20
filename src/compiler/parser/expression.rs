use crate::{
    common::position::Located,
    compiler::{
        lexer::token::{Token, TokenInfo},
        nodes::{
            ast::{Expression, Parameter, RawExpression, RawParameter},
            parser::{ParsingDelimiter, ParsingNode},
            shared::ArithmethicOperator,
        },
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
                Token::OpenBlock,
                Token::CloseBlock,
                Token::Plus,
                Token::Minus,
                Token::Asterisk,
                Token::ForwardSlash,
                Token::Integer(String::new()),
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
                    self.push_state(
                        ParsingDelimiter::Function {
                            name,
                            parameters,
                            return_type,
                        },
                        position,
                    );
                }
                Token::Return => {
                    self.push_state(ParsingDelimiter::Return, token.position);
                }
                Token::OpenBlock => {
                    self.push_state(ParsingDelimiter::StartBlock, token.position);
                }
                Token::CloseBlock => {
                    self.handle_delimiter(token)?;
                    if self.states.len() == 0 {
                        break;
                    }
                }
                Token::Integer(integer) => {
                    self.push_state(
                        ParsingNode::Expression(RawExpression::Integer(integer)),
                        token.position,
                    );
                }
                Token::Plus
                | Token::Minus
                | Token::ForwardSlash
                | Token::Asterisk
                | Token::Percent => {
                    use ArithmethicOperator::*;
                    let operator = match &token.raw {
                        Token::Plus => Plus,
                        Token::Minus => Minus,
                        Token::Asterisk => Multiply,
                        Token::ForwardSlash => Division,
                        Token::Percent => Remainder,
                        _ => unreachable!(),
                    };

                    self.push_state(ParsingNode::ArithmeticOperator(operator), token.position);
                }
                _ => todo!(),
            }
        }

        // Ok(expressions.pop())
        todo!()
    }
    pub fn handle_delimiter(&mut self, token: TokenInfo) -> DiagnosticResult<Expression> {
        println!("{:#?}", self.states);

        let state = match self.states.pop() {
            Some(e) => e,
            None => {
                return Err(DiagnosticData::basic(
                    format!("Missing closing delimiter '{}'", token.raw),
                    self.path(),
                ))
            }
        };

        match state.raw {
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
