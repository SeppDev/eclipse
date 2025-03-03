use crate::{
    common::position::{Located, PositionRange},
    compiler::{
        lexer::token::Token,
        nodes::{ast::Node, parser::ParserState},
    },
    diagnostics::{DiagnosticData, DiagnosticResult, Diagnostics},
};

use super::Parser;

impl Parser {
    pub fn start_parse(&mut self) -> DiagnosticResult<Node> {
        let state = loop {
            match self.next_node()? {
                Some(s) => break s,
                None => continue,
            }
            // println!("{:#?}", self.stack);
        };
        println!("{:#?}", state.raw);
        todo!()
    }

    fn next_node(&mut self) -> DiagnosticResult<Option<Located<ParserState>>> {
        let token = self.expect(vec![
            Token::Function,
            Token::OpenBlock,
            Token::CloseBlock,
            Token::VariableDecl,
            Token::Return,
            Token::Plus,
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

        let is_expression = raw.is_expression();

        let state = Located::new(raw, token.position);
        if is_expression {
            if let Some(node) = self.stack.last_mut() {
                if let Some(body) = node.raw.expression_body() {
                    body.push(state);
                    return Ok(None);
                }
            }
        }

        self.stack.push(state);

        Ok(None)
    }
    fn finish_expression(&mut self) -> DiagnosticResult<()> {
        Ok(())
    }
    fn finish_block(
        &mut self,
        position: PositionRange,
    ) -> DiagnosticResult<Option<Located<ParserState>>> {
        self.finish_expression()?;

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
            Some(l) => l,
            None => return Ok(Some(block)),
        };

        let body = last.raw.block().expect("Expected block");
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
