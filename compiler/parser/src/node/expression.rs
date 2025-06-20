use common::{
    layout::ast::{Node, RawNode},
    lexer::token::{Token, TokenKind},
};
use diagnostics::DiagnosticResult;

use crate::Parser;

use TokenKind::*;

impl Parser {
    pub fn expect_expression(&mut self) -> DiagnosticResult<Node> {
        self.parse_expression(0)
    }
    pub fn get_expression(&mut self) -> DiagnosticResult<Option<Node>> {
        if self.peek().kind.is_expression_start() {
            return Ok(None);
        }
        Ok(Some(self.parse_expression(0)?))
    }
    fn expect_raw_base_expression(&mut self) -> DiagnosticResult<RawNode> {
        let info = self.next()?;

        let raw = match info.kind {
            While => self.parse_while()?,
            Loop => self.parse_loop()?,
            Function => self.parse_function()?,
            Return => self.parse_return()?,
            Break => self.parse_continue()?,
            Continue => self.parse_break()?,
            Var => self.parse_variable_decl()?,
            OpenCurlyBracket => self.parse_block()?,
            Integer => RawNode::Integer(info.string),
            Float => RawNode::Float(info.string),
            Boolean => RawNode::Bool(info.string == "true"),
            String => RawNode::String(info.string),
            Minus => RawNode::Minus(self.expect_base_expression()?.into()),
            Identifier if self.peek().kind == DoubleColon => {
                let mut path: Vec<Identifier> = vec![info.into()];
                while self.peek().kind == DoubleColon {
                    let ident = self.expect_identifier()?;
                    path.push(ident.into());
                }

                RawNode::Path(path)
            }
            Identifier if self.peek().kind.is_equals_operation() => {
                self.parse_set_operation(info)?
            }
            Identifier => RawNode::Identifier(info.string),
            OpenParen => {
                let mut items = self.expect_arguments(CloseParen)?;
                match items.len() {
                    1 => RawNode::Wrapped(Some(Box::new(items.pop().unwrap()))),
                    0 => RawNode::Wrapped(None),
                    _ => RawNode::Tuple(items),
                }
            }
            _ if info.kind.is_modifier() => self.expect_modifiers_node()?,
            _ => {
                return DiagnosticData::error()
                    .title(format!("Expected expression, got: {:?}", info.kind))
                    .position(info.position)
                    .to_err();
            }
        };

        return Ok(raw);
    }
    fn expect_base_expression(&mut self) -> DiagnosticResult<Node> {
        let start = self.start();
        let raw = self.expect_raw_base_expression()?;

        let mut node = self.located(raw, start);

        while self.next_if(|t| t.kind == OpenParen)?.is_some() {
            let start = self.last_position.start;
            let arguments = self.expect_arguments(CloseParen)?;
            node = self.located(RawNode::Call(node.into(), arguments), start);
        }

        Ok(node)
    }
    fn make_expression(&mut self, left: Node, right: Node, info: Token) -> RawNode {
        let kind = info.kind;
        let raw = match kind {
            _ if kind.is_arithmetic_operator() => RawNode::ArithmethicOperation {
                left: left.into(),
                right: right.into(),
                operator: kind.try_into().unwrap(),
            },
            _ if kind.is_compare_operator() => RawNode::CompareOperation {
                left: left.into(),
                right: right.into(),
                operator: kind.try_into().unwrap(),
            },
            Dot => RawNode::Field(left.into(), right.into()),
            OpenParen => todo!(),
            _ => todo!(),
        };
        raw
    }
    fn parse_expression(&mut self, min_bp: u16) -> DiagnosticResult<Node> {
        let mut left = self.expect_base_expression()?;

        loop {
            let info = self.peek();
            match info.kind {
                Dot => {}
                _ if info.kind.is_operator() => {}
                _ => break,
            }

            let start = self.start();
            let info = self.next()?;

            let bp = binding_power(&info.kind);
            if bp.left < min_bp {
                break;
            }

            let right = self.parse_expression(bp.right)?;
            let raw = self.make_expression(left, right, info);
            left = self.located(raw, start)
        }

        Ok(left)
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

pub struct BindingPower {
    left: u16,
    right: u16,
}
impl BindingPower {
    pub fn new(left: u16, right: u16) -> Self {
        Self { left, right }
    }
}

pub fn binding_power(value: &TokenKind) -> BindingPower {
    use TokenKind::*;

    match self {
        Dot => BindingPower::new(101, 100),
        Asterisk | ForwardSlash => BindingPower::new(70, 71),
        Plus | Minus => BindingPower::new(60, 61),

        LessThan | LessThanOrEquals | GreaterThan | GreaterThanOrEquals => {
            BindingPower::new(40, 41)
        }
        Compare | NotEquals => BindingPower::new(39, 40),

        t => panic!("Unkown operator: {:?}", t),
    }
}
