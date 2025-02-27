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
    // pub fn _parse_node(&mut self) -> DiagnosticResult<Node> {
    // let mut active_stack: Vec<Located<ParserState>> = Vec::new();

    // let state = loop {
    //     if let Some(token) = self.next_if_eq(Token::CloseBlock) {
    //         let state = match active_stack.pop() {
    //             Some(s) => s,
    //             None => return Err(DiagnosticData::basic("Missing delimiter", self.path())),
    //         };

    //         if active_stack.len() == 0 {
    //             break state;
    //         }
    //     }

    //     let node = self.handle_token()?;
    //     self.handle_node(&mut active_stack, node)?;
    // };

    // let raw = match state.raw {
    //     ParserState::Function {
    //         name,
    //         parameters,
    //         return_type,
    //         body,
    //     } => RawNode::Function {
    //         name,
    //         parameters,
    //         return_type,
    //         body,
    //     },
    //     _ => todo!(),
    // };

    // Ok(Node::new(raw, state.position))
    // }
    // pub fn handle_node(&mut self, node: Located<ParserState>) -> DiagnosticResult<()> {
    //     match node.raw {
    //         ParserState::Function { .. } | ParserState::Block(..) => {
    //             stack.push(node);
    //             return Ok(());
    //         }
    //         _ => {}
    //     };

    //     let active = stack.last_mut().unwrap();
    //     let block = match &mut active.raw {
    //         ParserState::Function { body, .. } | ParserState::Block(body) => body,
    //         _ => todo!("{active:#?}"),
    //     };

    //     let expression = match node.raw {
    //         ParserState::Expression(expr) => expr,
    //         ParserState::Return => RawNode::Return(None),
    //         ParserState::Block(body) => RawNode::Block(body),
    //         _ => todo!("{node:#?}"),
    //     };

    //     block.push(Located::new(expression, node.position));
    // Ok(())
    // }

    pub fn start_parse(&mut self) -> DiagnosticResult<Node> {
        self.stack.clear();

        let node = loop {
            let state = self.next_state()?;
            todo!()
        };

        todo!()
    }

    fn next_state(&mut self) -> DiagnosticResult<Located<ParserState>> {
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
            Token::CloseBlock => self.finish_block(end),
            Token::OpenBlock => ParserState::Block(Vec::new()),
            Token::Return => ParserState::Return,
            Token::Function => self.start_function()?,
            Token::VariableDecl => self.start_var_decl()?,
            Token::Integer(_) => self.start_expression(token.raw)?,
            _ => unreachable!("{token:#?}"),
        };

        Ok(Located::new(raw, token.position))
    }
    fn finish_block(&mut self) -> DiagnosticResult<()> {
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
