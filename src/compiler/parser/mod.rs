use std::collections::HashMap;

use super::{
    ast,
    diagnostics::DiagnosticResult,
    lexer::{token::TokenInfo, tokenize},
    CompilerCtx,
};
use crate::{common::position::PositionRange, compiler::Path, FILE_EXTENSION};

mod common;
mod imports;
mod modules;
mod node;

#[derive(Debug)]
pub struct ParsedModule {
    pub imports: Vec<Path>,
    pub body: Vec<ast::Node>,
}

#[derive(Debug, Default)]
pub struct ParsedModules {
    pub files: HashMap<Path, ParsedModule>,
}

impl CompilerCtx {
    pub fn parse(&mut self) -> ParsedModules {
        let mut parsed = ParsedModules::default();
        let mut paths = Vec::new();
        let main_path = Path::new()
            .extend_single("src")
            .extend_single("main")
            .extension(FILE_EXTENSION);

        paths.push(main_path);

        while let Some(relative_path) = paths.pop() {
            let result = self.parse_relative(&relative_path);
            let diagnostics = self.diagnostics.file(&relative_path);

            let file = match result {
                Ok(f) => f,
                Err(err) => {
                    diagnostics.extract_error(err);
                    continue;
                }
            };

            for import in &file.imports {
                paths.push(import.clone());
            }

            parsed.files.insert(relative_path, file);
        }

        parsed
    }
    pub fn parse_relative(&self, relative_path: &Path) -> DiagnosticResult<ParsedModule> {
        self.message(format!("Parsing: {relative_path}"));

        let source = self.fs_read(&relative_path).unwrap();
        let mut parser = Parser::new(&source)?;
        let nodes = parser.parse()?;

        let mut imports: Vec<Path> = Vec::new();
        let mut body = Vec::with_capacity(nodes.len());

        for node in nodes {
            let name = match node.raw {
                ast::RawNode::Import(name) => name.raw,
                _ => {
                    body.push(node);
                    continue;
                }
            };
            let path = self.resolve_import(node.position, &relative_path, &name)?;
            imports.push(path);
        }

        let file = ParsedModule { body, imports };

        Ok(file)
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
