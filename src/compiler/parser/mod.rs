use std::{borrow::Borrow, collections::HashMap};

use super::{
    diagnostics::{DiagnosticData, DiagnosticResult, DiagnosticsFile},
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
mod imports;
mod node;

#[derive(Debug)]
pub struct ParsedFile {
    pub imports: Vec<Path>,
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
        let main_path = Path::new()
            .join("src")
            .join("main")
            .extension(FILE_EXTENSION);

        paths.push(main_path);

        while let Some(relative_path) = paths.pop() {
            let full_path = self.resolve_path(relative_path.clone());
            let mut diagnostics = DiagnosticsFile::new(relative_path.clone());

            diagnostics.then(|| -> DiagnosticResult {
                let source = match self.files.fs_read(&full_path.as_path_buf()) {
                    Ok(s) => s,
                    Err(_) => todo!(),
                };
                self.files.cache(full_path, source);

                let file = self.parse_relative(relative_path.clone())?;
                parsed.files.insert(relative_path, file);
                Ok(())
            });

            self.diagnostics.insert(diagnostics);
        }

        parsed
    }

    pub fn parse_relative(&self, relative_path: Path) -> DiagnosticResult<ParsedFile> {
        self.message(format!("Parsing: {relative_path}"));

        let full_path = self
            .resolve_path(relative_path.clone())
            .extension(FILE_EXTENSION);

        let source = self.files.from_cache(&full_path).unwrap();

        let mut parser = Parser::new(source)?;
        let body = parser.parse()?;

        let imports: Vec<Path> = Vec::new();
        for node in &body {
            let name = match &node.raw {
                RawNode::Import(name) => &name.raw,
                _ => continue,
            };
            let _path = self.handle_import(&relative_path, name)?;
            todo!()
        }

        Ok(ParsedFile { imports, body })
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

// fn parse_file(
//     &mut self,
//     relative_path: Path,
//     paths: &mut Vec<Path>,
// ) -> DiagnosticData<ParsedFile> {
//     let nodes = self.parse_relative(path.)?;
//     let mut body = Vec::with_capacity(nodes.len());
//     let mut imports = HashMap::new();

//     for node in nodes {
//         if let RawNode::Import(import) = node.raw {
//             let path = self.handle_import(&path, &import.raw)?;
//             let full_path = self.resolve_path(path.clone());
//             let source = self.files.fs_read(&full_path.as_path_buf()).unwrap();

//             self.files.cache(path.clone(), source);
//             imports.insert(import.raw, path.clone());
//             paths.push(path);
//             continue;
//         }
//         body.push(node);
//     }

//     let file = ParsedFile { imports, body };
//     parsed.files.insert(path, file);
// }
