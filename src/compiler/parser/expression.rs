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
            Token::If => self.start_conditional()?,

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
        let last = match self.stack.last_mut() {
            Some(l) if l.is_node() => l,
            _ => {
                self.stack.push(state);
                return Ok(None);
            }
        };

        if let Err(state) = last.insert(state) {
            self.stack.push(state);
        }

        Ok(None)
    }
    fn close_block(&mut self, position: PositionRange) -> DiagnosticResult<Located<ParserState>> {
        let mut block = match self.stack.pop() {
            Some(b) if b.is_block() => b,
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

        let body = if let ParserState::OpenBlock { body } = block.raw {
            body
        } else {
            unreachable!();
        };

        block.raw = ParserState::Block { body };

        return Ok(block);
    }
    fn finish_block(
        &mut self,
        position: PositionRange,
    ) -> DiagnosticResult<Option<Located<ParserState>>> {
        self.finish_statement()?;

        let block = self.close_block(position)?;

        let last_mut = match self.stack.last_mut() {
            Some(l) => l,
            None => return Ok(Some(block)),
        };

        if let Err(state) = last_mut.insert(block) {
            todo!("{state:?}");
        }

        return Ok(None);
    }
    pub fn finish_statement(&mut self) -> DiagnosticResult<()> {
        let last = match self.stack.last() {
            Some(last) if !last.is_block() => self.stack.pop().unwrap(),
            _ => return Ok(()),
        };
        // println!("~~~~~~~~~~~~~~~~~~~~");
        // println!("{last:#?}");
        // println!("{:#?}", self.stack);

        let node = self.stack.last_mut().unwrap();
        node.insert(last).unwrap();

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
            Token::ExclamationMark | Token::Compare => {
                use crate::compiler::nodes::shared::Operator::*;
                let operator = match &token {
                    Token::ExclamationMark => Not,
                    Token::Compare => Compare,
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
