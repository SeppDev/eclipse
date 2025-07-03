use ::common::{constants::FILE_EXTENSION, path::Path, position::PositionRange};
use context::CompilerCtx;
use diagnostics::DiagnosticResult;
use lexer::token::Token;
use lexer::tokenize;
use modules::{ASTModule, ASTModules};
use syntax::ast;

mod common;
mod imports;
mod modules;

pub fn parse() -> ASTModules {
    let mut parsed = ASTModules::default();
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

impl<'a> Parser<'a> {
    pub fn parse_relative(&self, relative_path: &Path) -> DiagnosticResult<ASTModule> {
        // self.message(format!("Parsing: {relative_path}"));

        let source = self.fs_read(&relative_path).unwrap();
        let mut parser = Parser::new(&source)?;
        let nodes = parser.parse()?;

        let mut imports: Vec<Path> = Vec::new();
        let mut body = Vec::with_capacity(nodes.len());

        for nod in nodes {
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

        let file = ASTModule { body, imports };

        Ok(file)
    }
}

pub struct Parser<'a> {
    compiler: &'a mut CompilerCtx,
    tokens: Vec<Token>,
    last_position: PositionRange,
}
impl<'a> Parser<'a> {
    pub fn new(compiler: &mut CompilerCtx, source: &String) -> DiagnosticResult<Self> {
        let mut tokens = tokenize(source)?;
        tokens.reverse();

        Ok(Self {
            tokens,
            last_position: PositionRange::default(),
        })
    }
}
