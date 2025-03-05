use crate::{
    common::position::{Located, PositionRange},
    compiler::{
        lexer::token::Token,
        nodes::{ast::Node, parser::ParserState},
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

use super::Parser;

impl Parser {
    pub fn start_parse(&mut self) -> DiagnosticResult<Located<ParserState>> {
        let state = loop {
            // println!("{:#?}", self.stack);
            match self.next_node()? {
                Some(s) => break s,
                None => continue,
            }
        };
        Ok(state)
    }

    fn next_node(&mut self) -> DiagnosticResult<Option<Located<ParserState>>> {
        let token = self.expect(vec![
            Token::Function,
            Token::OpenBlock,
            Token::CloseBlock,
            Token::VariableDecl,
            Token::Return,
            Token::Plus,
            Token::Minus,
            Token::Integer(String::new()),
            Token::Float(String::new()),
            Token::Identifier(String::new()),
        ])?;

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
        let node = match self.stack.last() {
            Some(last) if !last.raw.is_block() => self.stack.pop(),
            _ => None,
        };

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

        let last = match self.stack.last_mut() {
            Some(l) if l.raw.is_block() => l,
            _ => {
                let body = block.raw.block().unwrap();
                if let Some(state) = node {
                    body.push(state);
                }
                return Ok(Some(block));
            }
        };

        let body = last.raw.block().unwrap();
        if let Some(state) = node {
            body.push(state);
        }
        body.push(block);

        Ok(None)
    }
    // pub fn finish_statement(&mut self) -> DiagnosticResult<ParserState> {}
    pub fn start_block(&mut self) -> DiagnosticResult<ParserState> {
        Ok(ParserState::Block(Vec::new()))
    }
    pub fn start_return(&mut self) -> DiagnosticResult<ParserState> {
        Ok(ParserState::Return(Vec::new()))
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
            _ => todo!("{token:#?}"),
        };
        Ok(raw)
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
