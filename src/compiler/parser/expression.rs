use crate::{
    common::position::{Located, PositionRange},
    compiler::{
        lexer::token::Token,
        nodes::{
            ast::{Parameter, RawParameter},
            parser::ParserState,
        },
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

use super::Parser;

impl Parser {
    pub fn start_parse(&mut self) -> DiagnosticResult<Located<ParserState>> {
        let state = loop {
            println!("{}", ParserState::to_string_vec(&self.stack));
            println!("----");
            match self.next_node()? {
                Some(s) => break s,
                None => continue,
            }
        };
        Ok(state)
    }

    fn next_node(&mut self) -> DiagnosticResult<Option<Located<ParserState>>> {
        let token = self.next()?;

        let raw = match token.raw {
            Token::OpenBlock => self.start_block()?,
            Token::Return => self.start_return()?,
            Token::Function => self.start_function()?,
            Token::VariableDecl => self.start_var_decl()?,

            Token::CloseBlock => return self.finish_block(token.position),
            _ => self.start_expression(token.raw)?,
        };

        let state = Located::new(raw, token.position);
        self.handle_state(state)
    }
    fn handle_state(
        &mut self,
        state: Located<ParserState>,
    ) -> DiagnosticResult<Option<Located<ParserState>>> {
        if state.raw.is_node() {
            match self.stack.last() {
                Some(last) if last.raw.is_node() => {
                    let last = self.stack.pop().unwrap();
                    let block = self.stack.last_mut().unwrap();

                    if let Some(block) = block.raw.block() {
                        block.push(last);
                    }
                }
                _ => {}
            }
        }

        let last = match self.stack.last_mut() {
            Some(l) if l.raw.expects_expression() && state.raw.is_expression() => l,
            Some(l) if state.raw.is_operator() => l,
            _ => {
                self.stack.push(state);
                return Ok(None);
            }
        };

        if let Some(body) = last.raw.node_body() {
            body.push(state);
            return Ok(None);
        }

        self.stack.push(state);

        Ok(None)
    }
    fn finish_block(
        &mut self,
        position: PositionRange,
    ) -> DiagnosticResult<Option<Located<ParserState>>> {
        self.finish_statement()?;

        let mut block = match self.stack.pop() {
            Some(b) if b.raw.is_block() => b,
            _ => {
                return Err(DiagnosticData::new(
                    "Expected block",
                    self.path(),
                    "",
                    position,
                ))
            }
        };

        block.position.set_end(position.end);

        let body = match self.stack.last_mut() {
            Some(l) if l.raw.is_block() => l.raw.block().unwrap(),
            Some(l) if l.raw.expects_expression() => l.raw.node_body().unwrap(),
            None => return Ok(Some(block)),
            _ => unreachable!(),
        };

        body.push(block);

        Ok(None)
    }
    pub fn finish_statement(&mut self) -> DiagnosticResult<()> {
        let node = match self.stack.last() {
            Some(last) if !last.raw.is_block() => self.stack.pop().unwrap(),
            _ => return Ok(()),
        };

        let block = match self.stack.last_mut() {
            Some(state) if state.raw.is_block() => state.raw.block().unwrap(),
            // Some(state) if state.raw.expects_expression() => state.raw.node_body().unwrap(),
            _ => {
                self.stack.push(node);
                return Ok(());
            }
        };

        block.push(node);

        Ok(())
    }
    pub fn start_expression(&mut self, token: Token) -> DiagnosticResult<ParserState> {
        let raw = match token {
            Token::Integer(int) => ParserState::Integer(int),
            Token::Float(float) => ParserState::Float(float),
            Token::Identifier(identfier) => ParserState::Identifier(identfier),
            Token::Plus | Token::Minus | Token::ForwardSlash | Token::Asterisk | Token::Percent => {
                use crate::compiler::nodes::shared::ArithmethicOperator::*;
                let operator = match &token {
                    Token::Plus => Plus,
                    Token::Minus => Minus,
                    Token::Asterisk => Multiply,
                    Token::ForwardSlash => Division,
                    Token::Percent => Remainder,
                    _ => unreachable!(),
                };
                ParserState::ArithmeticOperator(operator)
            }
            Token::ExclamationMark => {
                use crate::compiler::nodes::shared::Operator::*;
                let operator = match &token {
                    Token::ExclamationMark => Not,
                    _ => unreachable!(),
                };
                ParserState::Operator(operator)
            }
            _ => todo!("{token:#?}"),
        };
        Ok(raw)
    }
    pub fn parse_parmeters(&mut self) -> DiagnosticResult<Vec<Parameter>> {
        let mut parameters = Vec::new();

        loop {
            if self.next_if_eq(Token::CloseParen)?.is_some() {
                break;
            }

            let reference = self.next_if_eq(Token::Ampersand)?;
            let mutable = self.next_if_eq(Token::Mutable)?;
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
// Token ::Plus | Token::Minus | Token::ForwardSlash | Token::Asterisk | Token::Percent => {
//     use ArithmethicOperator::*;
//     let operator = match &token.raw {
//         Token::Plus => Plus,
//         Token::Minus => Minus,
//         Token::Asterisk => Multiply,
//         Token::ForwardSlash => Division,
//         Token::Percent => Remainder,
//         _ => unreachable!(),
//     };

//     states.push(ParsingNode::ArithmeticOperator(operator), token.position)
