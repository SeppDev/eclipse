use std::{path::PathBuf, process::exit};

use crate::compiler::parser::{Expression, ExpressionInfo, Node, NodeInfo};

use super::{Location, Token, TokenInfo};
use std::{iter::Peekable, vec::IntoIter};

#[derive(Debug)]
pub struct Tokens {
    lines: Vec<String>,
    file_path: PathBuf,

    current: Option<TokenInfo>,
    starts: Vec<TokenInfo>,
    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl Tokens {
    pub fn new(file_path: PathBuf, tokens: Vec<TokenInfo>, lines: Vec<String>) -> Self {
        return Self {
            file_path,
            starts: Vec::new(),
            current: None,
            lines,
            tokens: tokens.into_iter().peekable(),
        };
    }
    pub fn throw_error<T: ToString, E: ToString>(&mut self, message: T, notice: E) -> ! {
        let current = self.current.clone().unwrap();
        let location = &current.location;
        let line = match self.lines.get(location.lines.start - 1) {
            Some(s) => s,
            None => panic!("Could not find: {:?}", location),
        };

        println!("error: {}", message.to_string());
        println!(
            "  --> {}:{}:{}",
            self.file_path.to_string_lossy(),
            location.lines.start,
            location.columns.start
        );

        println!("  |");
        println!("  | {}", line);
        println!(
            "  | {}{} {}",
            " ".repeat(location.columns.start - 1),
            "^".repeat(location.columns.end - current.location.columns.start),
            notice.to_string()
        );
        exit(1)
    }
    pub fn finish(mut self) {
        if self.starts.len() > 0 {
            println!("{:#?}", self.starts);
            panic!("Failed to finish: {:#?}", self.start())
        }
    }
    pub fn pop_start(&mut self) -> TokenInfo {
        self.starts.pop().unwrap()
    }
    pub fn push_start(&mut self, token: &TokenInfo) {
        self.starts.push(token.clone());
    }
    pub fn create_node(&mut self, node: Node) -> NodeInfo {
        let start = self.starts.pop().unwrap_or_else(|| {
            panic!("No starting node for: {:#?}", node);
        });
        let current = self.current.clone().unwrap().location;

        println!("CREATING NODE: {:#?} WITH {:#?}", node, start);
        let location = Location::new(
            start.location.lines.start..current.lines.start,
            start.location.columns.start..current.columns.start,
        );
        NodeInfo { node, location }
    }
    pub fn create_expression(&mut self, expression: Expression) -> ExpressionInfo {
        let start = self.starts.pop().unwrap_or_else(|| {
            panic!("No starting node for: {:#?}", expression);
        });
        let current = self.current.clone().unwrap().location;

        println!("CREATING EXPRESION: {:#?} WITH {:#?}", expression, start);

        let location = Location::new(
            start.location.lines.start..current.lines.start,
            start.location.columns.start..current.columns.start,
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
                // if info.token == Token::EndOfFile {
                //     self.throw_error("Early <EOF>", "")
                // }
                self.current = Some(info.clone());
                info
            }
            None => panic!(),
        }
    }
    pub fn peek(&mut self) -> &TokenInfo {
        return match self.tokens.peek() {
            Some(info) => info,
            None => todo!(),
        };
    }
    pub fn is_eof(&mut self) -> bool {
        return self.peek().token == Token::EndOfFile;
    }
}
