use crate::{
    common::position::{Located, PositionRange},
    compiler::{
        lexer::token::{Token, TokenInfo},
        nodes::{
            ast::{Node, Parameter, RawNode, RawParameter},
            parser::ParserState,
        },
    },
    diagnostics::DiagnosticResult,
};

use super::Parser;

impl Parser {
    pub fn parse_node(&mut self) -> DiagnosticResult<Node> {
        let mut active_stack: Vec<Located<ParserState>> = Vec::new();

        let state = loop {
            if let Some(token) = self.next_if_eq(Token::CloseBlock) {
                let state = active_stack.pop();
                if active_stack.len() == 0 {
                    break state;
                }
            }
            let node = self.handle_token()?;
            self.handle_node(&mut active_stack, node)?;
        };

        todo!("{state:#?}");
        // Ok(active_expression)
    }
    pub fn handle_node(
        &mut self,
        stack: &mut Vec<Located<ParserState>>,
        node: Located<ParserState>,
    ) -> DiagnosticResult<()> {
        match node.raw {
            ParserState::Function { .. } => {
                stack.push(node);
                return Ok(());
            }
            _ => {}
        };

        let active = stack.last_mut().unwrap();
        let block = match &mut active.raw {
            ParserState::Function { body, .. } | ParserState::Block(body) => body,
            _ => todo!("{active:#?}"),
        };

        let expression = match node.raw {
            ParserState::Expression(expr) => expr,
            ParserState::Return => RawNode::Return(None),
            ParserState::Block(body) => RawNode::Block(body),
            _ => todo!("{node:#?}"),
        };

        block.push(Located::new(expression, node.position));
        Ok(())
    }
    fn handle_token(&mut self) -> DiagnosticResult<Located<ParserState>> {
        let token = self.expect(vec![
            Token::Function,
            Token::OpenBlock,
            Token::Variable,
            Token::Return,
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::ForwardSlash,
            Token::Integer(String::new()),
        ])?;

        let raw = match token.raw {
            Token::Function => self.start_function()?,
            Token::OpenBlock => self.start_block()?,
            Token::Variable => self.start_var_decl()?,
            Token::Return => self.start_return()?,
            Token::Integer(_) => self.start_expression(token.raw)?,
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
            _ => unreachable!("{token:#?}"),
        };

        Ok(Located::new(raw, token.position))
    }
    pub fn start_block(&mut self) -> DiagnosticResult<ParserState> {
        Ok(ParserState::Block(Vec::new()))
    }
    pub fn start_expression(&mut self, token: Token) -> DiagnosticResult<ParserState> {
        let raw = match token {
            Token::Integer(int) => RawNode::Integer(int),
            _ => todo!("{token:#?}"),
        };
        Ok(ParserState::Expression(raw))
    }
    pub fn start_return(&mut self) -> DiagnosticResult<ParserState> {
        Ok(ParserState::Return)
    }
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
            body: Vec::new(),
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
