use std::{path::PathBuf, process::exit};

use super::TokenInfo;

#[derive(Debug)]
pub struct Tokens {
    lines: Vec<String>,
    file_path: PathBuf,
    starts: Vec<usize>,
    cursor: usize,
    tokens: Vec<TokenInfo>,
}
impl Tokens {
    pub fn new(file_path: PathBuf, tokens: Vec<TokenInfo>, lines: Vec<String>) -> Self {
        return Self {
            file_path,
            cursor: 0,
            starts: Vec::new(),
            lines,
            tokens,
        };
    }
    pub fn create_error<T: ToString, E: ToString>(&self, message: T, notice: E) -> ! {
        let current = self.current();
        let line = self.lines.get(current.lines.start - 1).unwrap();

        println!("error: {}", message.to_string());
        println!(
            "  --> {}:{}:{}",
            self.file_path.to_string_lossy(),
            current.lines.start,
            current.columns.start
        );

        println!("  |");
        println!("  | {}", line);
        println!(
            "  | {}{} {}",
            " ".repeat(current.columns.start - 1),
            "^".repeat(current.columns.end - current.columns.start),
            notice.to_string()
        );
        exit(1)
    }
    pub fn create_node(&mut self) {
        self.starts.pop();
    }
    pub fn current(&self) -> &TokenInfo {
        return match self.tokens.get(self.cursor - 1) {
            Some(info) => info,
            None => todo!(),
        };
    }
    pub fn start(&mut self) -> &TokenInfo {
        self.starts.push(self.cursor);
        let token = self.advance();
        token
    }
    pub fn advance(&mut self) -> &TokenInfo {
        return match self.tokens.get(self.cursor) {
            Some(info) => {
                self.cursor += 1;
                info
            }
            None => todo!(),
        };
    }
    pub fn peek(&self) -> &TokenInfo {
        return match self.tokens.get(self.cursor + 1) {
            Some(info) => info,
            None => todo!(),
        };
    }
}
