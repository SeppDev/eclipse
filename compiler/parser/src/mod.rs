use diagnostics::DiagnosticResult;
use lexer::Tokens;
use shared::{constants::FILE_EXTENSION, path::Path, position::PositionRange};
use syntax::ast;

use crate::context::CompilerCtx;

mod common;
mod node;

pub struct Parser<'a> {
    compiler: &'a mut CompilerCtx,
    root_path: Path,
    tokens: Tokens,
    last_position: PositionRange,
}
impl Parser<'_> {
    pub fn new(tokens: Tokens, root_path: Path, compiler: &'a mut CompilerCtx) -> Parser<'a> {
        Self {
            compiler,

            root_path,
            tokens,
            last_position: PositionRange::default(),
        }
    }
}

pub fn parse() -> ast::Module {
    let mut parsed = ast::Module::default();
    let mut paths = Vec::new();
    let main_path = Path::new()
        .extend_single("src")
        .extend_single("main")
        .extension(FILE_EXTENSION);

    paths.push(main_path);

    while let Some(relative_path) = paths.pop() {
        let result = parser.parse_relative(&relative_path);
        let diagnostics = parser.diagnostics.file(&relative_path);

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

        parsed.insert(relative_path, file);
    }

    parsed
}

impl<'a> Parser<'a> {
    pub fn parse_relative(&self, relative_path: &Path) -> DiagnosticResult<ast::Module> {
        // self.message(format!("Parsing: {relative_path}"));

        let source = self.fs_read(&relative_path).unwrap();
        let nodes = self.parse()?;

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

        let file = ast::Module { body, imports };

        Ok(file)
    }
}
