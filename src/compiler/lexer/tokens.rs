use std::path::PathBuf;

use crate::compiler::{
    errors::{CompileMessages, Location, Message, MessageKind},
    parser::{Expression, ExpressionInfo, Node, NodeInfo},
};

use super::{Token, TokenInfo};
use std::{iter::Peekable, vec::IntoIter};

#[derive(Debug)]
pub struct Tokens {
    lines: Vec<String>,
    file_path: PathBuf,
    errors: CompileMessages,

    current: Option<TokenInfo>,
    starts: Vec<TokenInfo>,
    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl Tokens {
    pub fn new(file_path: PathBuf, tokens: Vec<TokenInfo>, lines: Vec<String>) -> Self {
        return Self {
            file_path,
            errors: CompileMessages::new(),
            starts: Vec::new(),
            current: None,
            lines,
            tokens: tokens.into_iter().peekable(),
        };
    }
    pub fn throw_error<T: ToString, E: ToString>(
        &mut self,
        message: T,
        notice: E,
        location: Location,
    ) -> &mut Message {
        self.errors.create(
            MessageKind::Error,
            self.file_path.clone(),
            message,
            notice,
            location,
        )
    }
    pub fn finish(mut self) -> Vec<String> {
        if self.starts.len() > 0 {
            println!("{:#?}", self.starts);
            panic!("Failed to finish: {:#?}", self.start())
        }

        self.lines
    }
    pub fn pop_start(&mut self) -> TokenInfo {
        self.starts.pop().unwrap()
    }
    // pub fn push_start(&mut self, token: &TokenInfo) {
    //     self.starts.push(token.clone());
    // }
    pub fn create_node(&mut self, node: Node) -> NodeInfo {
        let start = self.starts.pop().unwrap_or_else(|| {
            panic!("No starting node for: {:#?}", node);
        });
        let current = self.current.clone().unwrap().location;
        let location = Location::new(
            start.location.lines.start..current.lines.end,
            start.location.columns.start..current.columns.end,
        );
        NodeInfo { node, location }
    }
    pub fn create_expression(&mut self, expression: Expression) -> ExpressionInfo {
        let start = self.starts.pop().unwrap_or_else(|| {
            panic!("No starting node for: {:#?}", expression);
        });
        let current = self.current.clone().unwrap().location;
        let location = Location::new(
            start.location.lines.start..current.lines.end,
            start.location.columns.start..current.columns.end,
        );

        ExpressionInfo {
            expression,
            location,
        }
    }
    pub fn start(&mut self) -> TokenInfo {
        let token = self.advance();
        self.starts.push(token.clone());
        token
    }
    pub fn advance(&mut self) -> TokenInfo {
        match self.tokens.next() {
            Some(info) => {
                self.current = Some(info.clone());
                info
            }
            None => panic!(),
        }
    }
    pub fn peek(&mut self) -> TokenInfo {
        return match self.tokens.peek() {
            Some(info) => info.clone(),
            None => todo!(),
        };
    }
    pub fn is_eof(&mut self) -> bool {
        return self.peek().token == Token::EndOfFile;
    }
}
