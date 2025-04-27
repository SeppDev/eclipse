use crate::compiler::{
    diagnostics::{DiagnosticData, DiagnosticResult},
    lexer::token::{TokenInfo, TokenKind},
    nodes::ast::{Identifier, Node, RawNode},
    parser::Parser,
};
use TokenKind::*;

impl Parser {
    pub fn expect_expression(&mut self) -> DiagnosticResult<Node> {
        if self.peek().kind.is_modifier() {
            let start = self.start();
            let value = self.expect_modifiers_expression()?;
            return Ok(self.located(value, start));
        }

        let mut node = self.expect_base_expression()?;
        if self.peek().kind.is_compare_operator() {
            node = self.handle_compare_operation(node)?;
            return Ok(node);
        }

        node = self.order_operations(
            node,
            |k| k.is_arithmetic_operator(),
            |kind, left, right| RawNode::ArithmethicOperation {
                left,
                right,
                operator: kind.into(),
            },
        )?;

        node = self.handle_compare_operation(node)?;

        return Ok(node);
    }
    pub fn handle_compare_operation(&mut self, node: Node) -> DiagnosticResult<Node> {
        self.order_operations(
            node,
            |k| k.is_compare_operator(),
            |kind, left, right| RawNode::CompareOperation {
                left,
                right,
                operator: kind.into(),
            },
        )
    }
    pub fn expect_base_expression(&mut self) -> DiagnosticResult<Node> {
        let start = self.start();
        let info = self.next()?;
        let node = self.to_expression(info)?;
        let mut node = self.located(node, start);

        while self.next_if_eq(Dot)?.is_some() {
            let start = self.start();
            let field = self.expect(&vec![Identifier, Integer])?;
            node = self.located(RawNode::Field(Box::new(node), field.into()), start);
        }

        while self.next_if_eq(OpenParen)?.is_some() {
            let start = self.start();
            let args = self.expect_arguments(CloseParen)?;
            node = self.located(RawNode::Call(Box::new(node), args), start);
        }
        Ok(node)
    }
    pub fn to_expression(&mut self, info: TokenInfo) -> DiagnosticResult<RawNode> {
        let raw = match info.kind {
            Float => RawNode::Float(info.string),
            Integer => RawNode::Integer(info.string),
            Boolean => RawNode::Bool(info.string == "true"),
            String => RawNode::String(info.string),
            Minus => RawNode::Minus(Box::new(self.expect_base_expression()?)),
            Identifier if self.peek().kind == DoubleColon => {
                let mut path: Vec<Identifier> = vec![info.into()];
                while self.next_if_eq(DoubleColon)?.is_some() {
                    let ident = self.expect_identifier()?.into();
                    path.push(ident);
                }
                RawNode::Path(path)
            }
            Function => self.parse_function()?,
            Identifier => RawNode::Identifier(info.string),
            Loop => RawNode::Loop(Box::new(self.expect_expression()?)),
            While => {
                let condition = Box::new(self.expect_expression()?);
                let body = Box::new(self.expect_expression()?);
                RawNode::While { condition, body }
            }
            If => {
                let condition = Box::new(self.expect_expression()?);
                let body = Box::new(self.expect_expression()?);
                RawNode::Conditional { condition, body }
            }
            OpenCurlyBracket => self.parse_block()?,
            OpenParen => {
                let mut items = self.expect_arguments(CloseParen)?;
                match items.len() {
                    1 => RawNode::Wrapped(Some(Box::new(items.pop().unwrap()))),
                    0 => RawNode::Wrapped(None),
                    _ => RawNode::Tuple(items),
                }
            }
            _ => {
                return DiagnosticData::error()
                    .title("Expected expression")
                    .position(info.position)
                    .to_err();
            }
        };

        Ok(raw)
    }
    pub fn expect_arguments(&mut self, delimiter: TokenKind) -> DiagnosticResult<Vec<Node>> {
        let mut arguments: Vec<Node> = Vec::new();

        if self.next_if_eq(&delimiter)?.is_some() {
            return Ok(arguments);
        }
        let expected = vec![delimiter.clone(), TokenKind::Comma];

        loop {
            let node = self.expect_node()?;
            arguments.push(node);

            match self.expect(&expected)?.kind {
                TokenKind::Comma => continue,
                kind if kind == delimiter => break Ok(arguments),
                _ => unreachable!(),
            }
        }
    }
}
