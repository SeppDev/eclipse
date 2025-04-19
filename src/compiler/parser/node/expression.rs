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
        let mut stack: Vec<Node> = Vec::new();
        let base = self.expect_base_expression(Some(info))?;
        


        Ok(base)
    }
    fn expect_base_expression(&mut self, info: Option<TokenInfo>) -> DiagnosticResult<RawNode> {
        let info = match info {
            Some(i) => i,
            _ => todo!(), // None => match self.next_if(|i| i.kind.is_expression())
        };

        let raw = match info.kind {
            TokenKind::Float => RawNode::Float(info.string),
            TokenKind::Integer => RawNode::Integer(info.string),
            TokenKind::Boolean => RawNode::Bool(info.string == "true"),
            TokenKind::Identifier => RawNode::Identifier(info.string),
            TokenKind::String => RawNode::String(info.string),
            _ => return Err(DiagnosticData::basic("Expected expression", self.path().clone())),
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
