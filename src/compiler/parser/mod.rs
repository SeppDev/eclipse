use super::{
    lexer::token::{TokenInfo, TokenKind},
    nodes::ast::Node,
    CompilerCtx,
};
use crate::{
    common::position::{Located, Position, PositionRange},
    diagnostics::{DiagnosticData, DiagnosticResult},
    FILE_EXTENSION,
};
use std::{borrow::Borrow, path::PathBuf};

mod node;
mod types;

mod reader;

const MAX_RECURSION: usize = 256;

pub struct Parser {
    tokens: Vec<TokenInfo>,
    path: PathBuf,
    start: Vec<Position>,
    last_position: PositionRange,
}
impl Drop for Parser {
    fn drop(&mut self) {
        if self.start.len() > 0 {
            let start = &self.start;
            println!("Failed to use all start positions: {start:#?}")
        };
    }
}
impl CompilerCtx {
    pub fn new_parser(&self, mut tokens: Vec<TokenInfo>, path: PathBuf) -> Parser {
        tokens.reverse();
        Parser {
            tokens,
            path,
            start: Vec::new(),
            last_position: PositionRange::default(),
        }
    }
}
impl Parser {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn start(&mut self) {
        self.start.push(self.peek().position.start);
        if self.start.len() >= MAX_RECURSION {
            panic!("Exceeded recursion limit: {MAX_RECURSION}")
        }
    }
    pub fn located<T>(&mut self, value: T) -> Located<T> {
        let start = self.start.pop().expect("Failed to create located value");
        let end = self.last_position.end;
        return Located::new(value, PositionRange::new(start, end));
    }
    pub fn is_eof(&self) -> bool {
        self.peek().kind == TokenKind::EndOfFile
    }
    pub fn next(&mut self) -> DiagnosticResult<TokenInfo> {
        let token = self.tokens.pop().unwrap();

        if token.kind == TokenKind::EndOfFile {
            return Err(DiagnosticData::new(
                "Expected token got <eof>",
                self.path.clone(),
                "",
                token.position,
            ));
        }
        self.last_position = token.position;

        Ok(token)
    }
    pub fn peek(&self) -> &TokenInfo {
        self.tokens.last().unwrap()
    }
    pub fn next_if(
        &mut self,
        func: impl FnOnce(&TokenInfo) -> bool,
    ) -> DiagnosticResult<Option<TokenInfo>> {
        let peeked = self.peek();
        if func(peeked) {
            return Ok(Some(self.next()?));
        }
        Ok(None)
    }
    pub fn next_if_eq(
        &mut self,
        kind: impl Borrow<TokenKind>,
    ) -> DiagnosticResult<Option<TokenInfo>> {
        self.next_if(|t| &t.kind == kind.borrow())
    }
    pub fn next_if_expected(
        &mut self,
        expected: &Vec<TokenKind>,
    ) -> DiagnosticResult<Option<TokenInfo>> {
        let peeked = self.peek();
        for t in expected {
            if &peeked.kind == t {
                return Ok(Some(self.next()?));
            }
        }
        Ok(None)
    }
    pub fn peek_expect(&self, expected: &Vec<TokenKind>) -> DiagnosticResult<&TokenInfo> {
        let peeked = self.peek();
        for t in expected.iter() {
            if &peeked.kind == t {
                return Ok(peeked);
            }
        }

        Err(DiagnosticData::new(
            format!(
                "Expected token(s): {}, got: '{:?}'",
                expected
                    .iter()
                    .map(|e| format!("'{e:?}'"))
                    .collect::<Vec<String>>()
                    .join(", "),
                peeked.kind
            ),
            self.path.clone(),
            "",
            peeked.position.clone(),
        ))
    }
    pub fn peek_found(&self, expected: &Vec<TokenKind>) -> Option<&TokenInfo> {
        let peeked = self.peek();
        for t in expected.iter() {
            if &peeked.kind == t {
                return Some(peeked);
            }
        }
        None
    }
    pub fn expect(&mut self, expected: &Vec<TokenKind>) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(&expected)?;
        self.next()
    }
    pub fn expect_single(&mut self, expected: TokenKind) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(&vec![expected])?;
        self.next()
    }
    pub fn peek_expect_single(&mut self, expected: TokenKind) -> DiagnosticResult<&TokenInfo> {
        self.peek_expect(&vec![expected])
    }
    pub fn expect_identifier(&mut self) -> DiagnosticResult<TokenInfo> {
        self.expect_single(TokenKind::Identifier)
    }
}

impl CompilerCtx {
    pub fn parse(&mut self) -> DiagnosticResult<()> {
        let mut to_tokenize = Vec::new();
        to_tokenize.push(PathBuf::from("src/main"));

        loop {
            let mut path = match to_tokenize.pop() {
                Some(p) => p,
                None => break,
            };
            path.set_extension(FILE_EXTENSION);
            let body = self.parse_tokens(&mut to_tokenize, path)?;
            println!("{:#?}", body);
        }

        Ok(())
    }
    pub(super) fn parse_tokens(
        &mut self,
        paths: &mut Vec<PathBuf>,
        current_path: PathBuf,
    ) -> DiagnosticResult<Vec<Node>> {
        let file_name = current_path.file_name().unwrap().to_str().unwrap();
        let is_module = file_name.starts_with("main") | file_name.starts_with("mod");

        self.message(format!("Lexer: {current_path:?}"));
        let source = self.read_relative(&current_path)?;
        let tokens = self.tokenize(source.as_str())?;

        let mut parser = self.new_parser(tokens, current_path.clone());
        let mut body = Vec::new();

        loop {
            if parser.is_eof() {
                break;
            }

            if parser.next_if_eq(TokenKind::Import)?.is_some() {
                let name = parser.expect_identifier()?;
                let path =
                    self.handle_import(&current_path, &is_module, file_name, &name.string)?;
                paths.push(path);
                continue;
            }

            let expression = parser.expect_node()?;
            body.push(expression);
        }

        Ok(body)
    }

    pub(super) fn handle_import(
        &self,
        current_path: &PathBuf,
        is_module: &bool,
        file_name: &str,
        name: &String,
    ) -> DiagnosticResult<PathBuf> {
        let paths: [PathBuf; 2] = if *is_module {
            [
                current_path.parent().unwrap().join(&name),
                current_path.parent().unwrap().join(&name).join("mod"),
            ]
        } else {
            [
                current_path.parent().unwrap().join(&file_name).join(&name),
                current_path
                    .parent()
                    .unwrap()
                    .join(&name)
                    .join(&file_name)
                    .join("mod"),
            ]
        };

        let mut found_paths: Vec<PathBuf> = Vec::with_capacity(2);
        for path in paths {
            let mut full_path = self.project_path.join(&path);
            full_path.set_extension(FILE_EXTENSION);

            if full_path.exists() {
                found_paths.push(path);
            }
        }

        if found_paths.len() == 0 {
            return Err(DiagnosticData::basic(
                "No valid path found",
                current_path.clone(),
            ));
        } else if found_paths.len() == 2 {
            return Err(DiagnosticData::basic(
                "Only one path can be active",
                current_path.clone(),
            ));
        }

        Ok(found_paths.pop().unwrap())
    }
}
