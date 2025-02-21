use crate::{
    common::position::{Located, PositionRange},
    compiler::{
        lexer::token::{Token, TokenInfo},
        nodes::{
            ast::{Expression, Parameter, RawExpression, RawParameter},
            parser::{IntoParsingState, ParsingDelimiter, ParsingNode, ParsingState, StartState},
            shared::ArithmethicOperator,
        },
    },
    diagnostics::DiagnosticResult,
};

use super::Parser;

pub struct ParserStates {
    states: Vec<StartState>,
    indent: usize
}
impl ParserStates {
    pub fn push_state<T: IntoParsingState>(&mut self, state: T, position: PositionRange) {
        self.states
            .push(StartState::new(state.into_state(), position))
    }
}

impl Parser {
    pub fn expect_expression(&mut self) -> DiagnosticResult<Expression> {
        loop {
            self.handle_token()?;

            todo!()
        }
    }
    fn handle_token(&mut self) -> DiagnosticResult<()> {
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
            Token::CloseBlock | Token::CloseParen => {
                let expression = self.handle_delimiter()?;
                self.push_state(expression.raw, expression.position);
            }
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
                ParsingDelimiter::Return, token.position
            }
            Token::OpenBlock => {
                self.push_state(ParsingDelimiter::StartBlock, token.position);
            }
            Token::Integer(integer) => {
                self.push_state(
                    ParsingNode::Expression(RawExpression::Integer(integer)),
                    token.position,
                );
            }
            Token::Plus | Token::Minus | Token::ForwardSlash | Token::Asterisk | Token::Percent => {
                use ArithmethicOperator::*;
                let operator = match &token.raw {
                    Token::Plus => Plus,
                    Token::Minus => Minus,
                    Token::Asterisk => Multiply,
                    Token::ForwardSlash => Division,
                    Token::Percent => Remainder,
                    _ => unreachable!(),
                };

                StartState::new(ParsingNode::ArithmeticOperator(operator).into(), token.position)
            }
            _ => todo!(),
        }
    }
    // pub fn handle_delimiter(&mut self) -> DiagnosticResult<Expression> {
    //     let mut nodes = Vec::new();
    //     let start = loop {
    //         let state = self.states.pop().unwrap();
    //         match state.raw {
    //             ParsingState::Delimiter(d) => break Located::new(d, state.position),
    //             ParsingState::Node(node) => nodes.push(node),
    //         }
    //     };

    //     let expression: Option<Expression> = None;

    //     let position = match &expression {
    //         Some(e) => e.position,
    //         None => start.position,
    //     };

    //     let raw = match start.raw {
    //         ParsingDelimiter::Return => RawExpression::Return(Box::new(expression)),
    //         _ => todo!(),
    //     };

    //     println!("{nodes:#?}");

    //     Ok(Expression::new(raw, position))
    // }
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
