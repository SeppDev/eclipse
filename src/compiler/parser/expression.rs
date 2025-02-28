use crate::{
    common::position::{Located, PositionRange},
    compiler::{
        lexer::token::{Token, TokenInfo},
        nodes::{
            ast::{Node, Parameter, RawNode, RawParameter},
            parser::ParserState,
        },
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

use super::Parser;

impl Parser {
    pub fn start_parse(&mut self) -> DiagnosticResult<Node> {
        let stack = Vec::new();
        let node = loop {
            let state = self.next_state()?;
            todo!()
        };

        todo!()
    }

    fn next_state(&mut self) -> DiagnosticResult<()> {
        let token = self.expect(vec![
            Token::Function,
            Token::OpenBlock,
            Token::CloseBlock,
            Token::VariableDecl,
            Token::Return,
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::ForwardSlash,
            Token::Integer(String::new()),
        ])?;

        let raw = match token.raw {
            // Token::SemiColon => self.finish_statement()?,
            Token::CloseBlock => self.finish_block(end)?,
            Token::OpenBlock => ParserState::Block(Vec::new()),
            Token::Return => ParserState::Return,
            Token::Function => self.start_function()?,
            Token::VariableDecl => self.start_var_decl()?,
            _ => self.start_expression(token.raw)?,
        };

        self.stack.push(Located::new(raw, token.position));
        Ok(())
    }
    fn finish_block(&mut self, end: TokenInfo) -> DiagnosticResult<ParserState> {
        let mut nodes = Vec::new();
        let start = loop {
            let state = match self.stack.pop() {
                Some(s) => s,
                None => {
                    return Err(DiagnosticData::new(
                        "No node for closing delimiter",
                        self.path(),
                        "",
                        end.position,
                    ))
                }
            };

            if let ParserState::Block(..) = state.raw {
                break state;
            }
            nodes.push(state);
        };

        let position = start.position.start.extend(end.position.end);

        while let Some(state) = nodes.pop() {
            let raw = match state.raw {
                ParserState::Return => RawNode::Return(None),
                _ => todo!(),
            };
            let node = Node::new(raw, state.position);
            body.push(node);
        }

        self.stack.push(Node::new(raw, position));
        todo!()
    }
    pub fn start_expression(&mut self, token: Token) -> DiagnosticResult<ParserState> {
        let raw = match token {
            Token::Integer(int) => RawNode::Integer(int),
            _ => todo!("{token:#?}"),
        };
        Ok(ParserState::Expression(raw))
    }
}
// Token::Plus | Token::Minus | Token::ForwardSlash | Token::Asterisk | Token::Percent => {
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
