use std::{borrow::Borrow, collections::HashMap};

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

#[derive(Debug)]
pub struct ParsedFile {
    pub imports: HashMap<String, Path>,
    pub body: Vec<Node>,
}

#[derive(Debug, Default)]
pub struct ParsedFiles {
    pub files: HashMap<Path, ParsedFile>,
}

impl CompilerCtx {
    pub fn parse(&mut self) -> DiagnosticResult<ParsedFiles> {
        let mut parsed = ParsedFiles::default();
        let mut paths = Vec::new();
        paths.push(Path::new().join("src").join("main"));

        while let Some(path) = paths.pop() {
            let nodes = self.parse_relative(path.clone())?;
            let mut body = Vec::with_capacity(nodes.len());
            let mut imports = HashMap::new();

            for node in nodes {
                if let RawNode::Import(import) = node.raw {
                    let path = self.handle_import(&path, &import.raw)?;
                    imports.insert(import.raw, path.clone());
                    paths.push(path);
                    continue;
                }
                body.push(node);
            }

            let file = ParsedFile { imports, body };
            parsed.files.insert(path, file);
        }

        Ok(parsed)
    }
    pub fn parse_relative(&mut self, mut relative_path: Path) -> DiagnosticResult<Vec<Node>> {
        relative_path.set_extension(FILE_EXTENSION);

        self.message(format!("Parsing: {relative_path}"));

        // let diagnostics = self.diagnostics.file(relative_path.clone());
        let full_path = self.resolve_path(relative_path);

        let msg = format!("Failed to find: {full_path}");
        let source = match self.files.from_cache(&full_path) {
            Some(s) => s,
            None => &self
                .files
                .fs_read(&full_path.as_path_buf())
                .expect(msg.as_str()),
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

            let node = self.top_level_expect()?;
            nodes.push(node);
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
    pub fn expect(&mut self, expected: &Vec<TokenKind>) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(&expected)?;
        self.next()
    }
    pub fn expect_single(&mut self, expected: TokenKind) -> DiagnosticResult<TokenInfo> {
        self.peek_expect(&vec![expected])?;
        self.next()
    }

    pub fn expect_identifier(&mut self) -> DiagnosticResult<TokenInfo> {
        self.expect_single(TokenKind::Identifier)
    }
}

impl CompilerCtx {
    fn handle_import(&self, current_relative_path: &Path, name: &str) -> DiagnosticResult<Path> {
        let file_name = current_relative_path.last().unwrap();
        let is_module = file_name == "mod" || file_name == "main";

        let parent = current_relative_path.parent();
        let expected_paths: [Path; 2] = if is_module {
            [parent.join(&name), parent.join(&name).join("mod")]
        } else {
            [
                parent.join(&file_name).join(&name),
                parent.join(&name).join(&file_name).join("mod"),
            ]
        };

        let mut found: Vec<Path> = Vec::with_capacity(2);
        for relative_path in &expected_paths {
            let mut full_path = self.resolve_path(relative_path.clone());
            full_path.set_extension(FILE_EXTENSION);

            if full_path.exists() {
                found.push(relative_path.to_owned());
            }
        }

        if found.len() > 1 {
            return DiagnosticData::error()
                .title(format!(
                    "Unresolved module, found two modules {expected_paths:?}"
                ))
                .to_err();
        }

        if let Some(path) = found.pop() {
            return Ok(path);
        }

        DiagnosticData::error()
            .title(format!(
                "Unresolved module, can't find module {expected_paths:?}"
            ))
            .to_err()
    }
}
