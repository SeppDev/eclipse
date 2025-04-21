use crate::{
    compiler::{
        lexer::token::{TokenInfo, TokenKind},
        nodes::{
            ast::{Node, RawNode},
            shared::Operator,
        },
        parser::Parser,
    },
    diagnostics::{DiagnosticData, DiagnosticResult},
};

#[derive(Debug)]
enum NodeKind {
    Expression(Node),
    Operator(TokenInfo),
}
impl Into<Node> for NodeKind {
    fn into(self) -> Node {
        match self {
            NodeKind::Expression(node) => node,
            _ => panic!("Invalid node kind"),
        }
    }
}

impl Parser {
    pub fn expect_expression(&mut self) -> DiagnosticResult<Node> {
        let node = self.expect_base_expression()?;
        if !self.peek().kind.is_operator() {
            return Ok(node);
        }

        let mut stack = vec![NodeKind::Expression(node)];
        while self.peek().kind.is_operator() {
            let operator = self.next()?;
            stack.push(NodeKind::Operator(operator));
            let expression = self.expect_base_expression()?;
            stack.push(NodeKind::Expression(expression));
        }
        let mut input = stack.into_iter();

        let mut holding_stack: Vec<TokenInfo> = Vec::new();
        let mut output: Vec<NodeKind> = Vec::new();

        while let Some(node) = input.next() {
            let operator = match node {
                NodeKind::Expression(node) => {
                    output.push(NodeKind::Expression(node));
                    continue;
                }
                NodeKind::Operator(operator) => operator,
            };

            let precedence = operator.kind.precedence();
            loop {
                match holding_stack.last() {
                    Some(l) if &l.kind.precedence() >= &precedence => {
                        let last = holding_stack.pop().unwrap();
                        output.push(NodeKind::Operator(last));
                    }
                    _ => {
                        holding_stack.push(operator);
                        break;
                    }
                }
            }
        }
        output.extend(
            holding_stack
                .into_iter()
                .map(|token| NodeKind::Operator(token)),
        );

        let mut output = output.into_iter();
        let mut solve_stack: Vec<NodeKind> = Vec::new();
        while let Some(node) = output.next() {
            let operator: Operator = match node {
                NodeKind::Expression(node) => {
                    solve_stack.push(NodeKind::Expression(node));
                    continue;
                }
                NodeKind::Operator(operator) => operator.into(),
            };

            let right: Box<Node> = Box::new(solve_stack.pop().unwrap().into());
            let left: Box<Node> = Box::new(solve_stack.pop().unwrap().into());
            let mut position = left.position;
            position.set_end(right.position.end);
            let result = match operator {
                Operator::Arithmetic(operator) => RawNode::ArithmethicOperation {
                    left,
                    right,
                    operator,
                },
                Operator::Comparison(operator) => RawNode::CompareOperation {
                    left,
                    right,
                    operator,
                },
            };

            solve_stack.push(NodeKind::Expression(Node::new(result, position)));
        }

        assert!(solve_stack.len() == 1);
        Ok(solve_stack.pop().unwrap().into())
    }
    pub fn expect_base_expression(&mut self) -> DiagnosticResult<Node> {
        self.start();
        let info = self.next()?;
        let node = self.to_expression(info)?;
        let mut node = self.located(node);

        loop {
            if self.next_if_eq(TokenKind::Dot)?.is_none() {
                break;
            }
            self.start();
            use TokenKind::*;
            let field = self.expect(&vec![Identifier, Integer])?;
            node = self.located(RawNode::Field(Box::new(node), field.into()));
        }

        Ok(node)
    }
    pub fn to_expression(&mut self, info: TokenInfo) -> DiagnosticResult<RawNode> {
        use TokenKind::*;

        let raw = match info.kind {
            Float => RawNode::Float(info.string),
            Integer => RawNode::Integer(info.string),
            Boolean => RawNode::Bool(info.string == "true"),
            String => RawNode::String(info.string),
            Minus => RawNode::MinusInteger(Box::new(self.expect_base_expression()?)),
            Identifier => RawNode::Identifier(info.string),
            OpenParen => {
                let mut items = self.expect_arguments(TokenKind::CloseParen)?;
                match items.len() {
                    1 => RawNode::Wrapped(Some(Box::new(items.pop().unwrap()))),
                    0 => RawNode::Wrapped(None),
                    _ => RawNode::Tuple(items),
                }
            }
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
