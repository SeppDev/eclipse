use crate::{
    compiler::{
        lexer::token::{TokenInfo, TokenKind},
        nodes::ast::{Node, RawNode},
        parser::Parser,
    },
    diagnostics::DiagnosticResult,
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
    pub fn order_operations<F, C>(
        &mut self,
        base: Node,
        is_operator: F,
        create_operation: C,
    ) -> DiagnosticResult<Node>
    where
        F: Fn(&TokenKind) -> bool,
        C: Fn(TokenKind, Box<Node>, Box<Node>) -> RawNode,
    {
        let mut stack = vec![NodeKind::Expression(base)];
        while is_operator(&self.peek().kind) {
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
            let kind = match node {
                NodeKind::Expression(node) => {
                    solve_stack.push(NodeKind::Expression(node));
                    continue;
                }
                NodeKind::Operator(operator) => operator.kind,
            };

            let right: Box<Node> = Box::new(solve_stack.pop().unwrap().into());
            let left: Box<Node> = Box::new(solve_stack.pop().unwrap().into());

            let mut position = left.position;
            position.set_end(right.position.end);
            let result = create_operation(kind, left, right);

            solve_stack.push(NodeKind::Expression(Node::new(result, position)));
        }

        assert!(solve_stack.len() == 1);
        Ok(solve_stack.pop().unwrap().into())
    }
}
