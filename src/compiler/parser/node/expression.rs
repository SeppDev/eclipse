use crate::{
    compiler::{
        lexer::token::{TokenInfo, TokenKind},
        nodes::ast::{Node, RawNode},
        parser::Parser,
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

impl Parser {
    pub fn parse_expression(&mut self, info: TokenInfo) -> DiagnosticResult<RawNode> {
        use TokenKind::*;

        let raw = match info.kind {
            Float => RawNode::Float(info.string),
            Integer => RawNode::Integer(info.string),
            Boolean => RawNode::Bool(info.string == "true"),
            String => RawNode::String(info.string),
            Minus => RawNode::MinusInteger(Box::new(self.expect_node()?)),
            Identifier => RawNode::Identifier(info.string),
            _ => {
                return Err(DiagnosticData::basic(
                    "Expected expression",
                    self.path().clone(),
                ))
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
