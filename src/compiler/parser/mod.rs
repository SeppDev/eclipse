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

mod common;
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
    pub fn parse(&mut self) -> ParsedFiles {
        let mut parsed = ParsedFiles::default();
        let mut paths = Vec::new();
        paths.push(Path::new().join("src").join("main"));

        while let Some(path) = paths.pop() {
            let diagnostics = self.diagnostics.file(path.clone());

            let nodes = self.parse_relative(path.clone())?;
            let mut body = Vec::with_capacity(nodes.len());
            let mut imports = HashMap::new();

            for node in nodes {
                if let RawNode::Import(import) = node.raw {
                    let path = self.handle_import(&path, &import.raw)?;
                    let full_path = self.resolve_path(path.clone());
                    let source = self.files.fs_read(&full_path.as_path_buf()).unwrap();

                    self.files.cache(path.clone(), source);
                    imports.insert(import.raw, path.clone());
                    paths.push(path);
                    continue;
                }
                body.push(node);
            }

            let file = ParsedFile { imports, body };
            parsed.files.insert(path, file);
        }

        parsed
    }
    pub fn parse_relative(&mut self, mut relative_path: Path) -> DiagnosticResult<Vec<Node>> {
        relative_path.set_extension(FILE_EXTENSION);

        self.message(format!("Parsing: {relative_path}"));

        let full_path = self.resolve_path(relative_path.clone());
        let source = self.files.from_cache(&full_path).unwrap();

        let mut parser = Parser::new(source)?;
        let nodes = parser.parse()?;

        Ok(nodes)
    }
}

#[derive(Default)]
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
            ..Default::default()
        })
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
            let mut relative_path = relative_path.to_owned();
            relative_path.set_extension(FILE_EXTENSION);

            let full_path = self.resolve_path(relative_path.clone());
            if full_path.exists() {
                found.push(relative_path);
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
