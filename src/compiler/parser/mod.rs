use std::collections::HashMap;

use super::{
    diagnostics::DiagnosticResult,
    lexer::{token::TokenInfo, tokenize},
    nodes::ast::{Node, RawNode},
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
    pub body: Vec<Node>,
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
            let result = self.parse_relative(relative_path.clone());
            let diagnostics = self.diagnostics.file(relative_path.clone());

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
    pub fn parse_relative(&self, relative_path: Path) -> DiagnosticResult<ParsedModule> {
        self.message(format!("Parsing: {relative_path}"));

        let source = self.fs_read(&relative_path).unwrap();
        let mut parser = Parser::new(&source)?;
        let body = parser.parse()?;

        let mut imports: Vec<Path> = Vec::new();

        for node in &body {
            let name = match &node.raw {
                RawNode::Import(name) => &name.raw,
                // RawNode::Modifiers(_, node) => todo!(),
                _ => continue,
            };
            let path = self.resolve_import(&relative_path, name)?;
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
