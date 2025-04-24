use std::borrow::Borrow;

use super::{
    diagnostics::{DiagnosticData, DiagnosticResult},
    lexer::{
        token::{TokenInfo, TokenKind},
        tokenize,
    },
    nodes::ast::{Node, RawNode},
    CompilerCtx,
};
use crate::{
    common::position::{LocatedAt, Position, PositionRange},
    compiler::Path,
    FILE_EXTENSION,
};

mod node;
mod reader;
mod types;

impl CompilerCtx {
    pub fn parse(&mut self) -> DiagnosticResult<Vec<Node>> {
        self.parse_relative(Path::new().join("src").join("main"))
    }
    pub fn parse_relative(&mut self, mut relative_path: Path) -> DiagnosticResult<Vec<Node>> {
        relative_path.set_extension(FILE_EXTENSION);

        self.message(format!("Parsing: {relative_path}"));

        let diagnostics = self.diagnostics.file(relative_path.clone());
        let full_path = self.resolve_path(relative_path);

        let msg = format!("Failed to find: {full_path}");
        let source = match self.files.from_cache(&full_path) {
            Some(s) => s,
            None => &self.files.fs_read(&full_path.into()).expect(msg.as_str()),
        };

        let mut parser = Parser::new(source)?;
        let nodes = parser.parse()?;

        Ok(nodes)
    }
}

pub struct Parser {
    tokens: Vec<TokenInfo>,
    last_position: PositionRange,
}
impl Parser {
    pub fn new(source: &String) -> DiagnosticResult<Self> {
        let mut tokens = tokenize(source)?;
        tokens.reverse();

        Ok(Self {
            tokens,
            last_position: PositionRange::default(),
        })
    }
    pub fn parse(&mut self) -> DiagnosticResult<Vec<Node>> {
        let mut nodes = Vec::new();

        loop {
            if self.is_eof() {
                break;
            }

            if let Some(node) = self.next_if_eq(TokenKind::Import)? {
                let name = self.expect_identifier()?;
                let mut position = node.position;
                position.set_end(name.position.end);

                nodes.push(Node::new(RawNode::Import(name.into()), position));
                continue;
            }

            let expression = self.expect_node()?;
            nodes.push(expression);
        }

        Ok(nodes)
    }
    pub fn start(&self) -> Position {
        self.peek().position.start
    }
    pub fn located<T>(&mut self, value: T, start: Position) -> LocatedAt<T> {
        let end = self.last_position.end;
        return LocatedAt::new(value, PositionRange::new(start, end));
    }
    pub fn is_eof(&self) -> bool {
        self.peek().kind == TokenKind::EndOfFile
    }
    pub fn next(&mut self) -> DiagnosticResult<TokenInfo> {
        let token = self.tokens.pop().unwrap();

        if token.kind == TokenKind::EndOfFile {
            return DiagnosticData::error()
                .title("Expected token got <eof>")
                .position(token.position)
                .to_err();
        }
        self.last_position = token.position;

        Ok(token)
    }
    pub fn peek(&self) -> &TokenInfo {
        self.tokens.last().unwrap()
    }
    pub fn peek_second(&self) -> &TokenInfo {
        self.tokens.get(self.tokens.len() - 2).unwrap()
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

        let title = format!(
            "Expected token(s): {}, got: '{:?}'",
            expected
                .iter()
                .map(|e| format!("'{e:?}'"))
                .collect::<Vec<String>>()
                .join(", "),
            peeked.kind
        );

        DiagnosticData::error()
            .title(title)
            .position(peeked.position.clone())
            .to_err()
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

// pub(super) fn handle_import(
//     &self,
//     current_path: &PathBuf,
//     is_module: &bool,
//     file_name: &str,
//     name: &String,
// ) -> DiagnosticResult<PathBuf> {
//     let paths: [PathBuf; 2] = if *is_module {
//         [
//             current_path.parent().unwrap().join(&name),
//             current_path.parent().unwrap().join(&name).join("mod"),
//         ]
//     } else {
//         [
//             current_path.parent().unwrap().join(&file_name).join(&name),
//             current_path
//                 .parent()
//                 .unwrap()
//                 .join(&name)
//                 .join(&file_name)
//                 .join("mod"),
//         ]
//     };

//     let mut found_paths: Vec<PathBuf> = Vec::with_capacity(2);
//     for path in paths {
//         let mut full_path = self.project_path.join(&path);
//         full_path.set_extension(FILE_EXTENSION);

//         if full_path.exists() {
//             found_paths.push(path);
//         }
//     }

//     if found_paths.len() == 0 {
//         return Err(DiagnosticData::basic(
//             "No valid path found",
//             current_path.clone(),
//         ));
//     } else if found_paths.len() == 2 {
//         return Err(DiagnosticData::basic(
//             "Only one path can be active",
//             current_path.clone(),
//         ));
//     }

//     Ok(found_paths.pop().unwrap())
// }
