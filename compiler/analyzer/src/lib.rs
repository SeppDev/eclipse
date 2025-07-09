use std::path::PathBuf;

use context::CompilerCtx;
use diagnostics::{DiagnosticData, DiagnosticResult};
use lexer::tokenize;
use parser::parse;
use syntax::ast;

mod semantics;
#[allow(unused)]
mod types;

struct Analyzer<'ctx> {
    compiler: &'ctx mut CompilerCtx,
}

pub fn analyze(compiler: &mut CompilerCtx, entry: PathBuf) -> DiagnosticResult {
    let modules = parse_modules(compiler, entry)?;
    panic!("{modules:#?}");

    Ok(())
}

pub fn parse_modules(compiler: &mut CompilerCtx, entry: PathBuf) -> DiagnosticResult<ast::Modules> {
    let mut to_parse: Vec<PathBuf> = Vec::new();
    to_parse.push(entry);

    let mut modules = ast::Modules::new();
    while let Some(relative_path) = to_parse.pop() {
        let source = match compiler.read(&relative_path) {
            Some(s) => s,
            None => return DiagnosticData::error().to_err(),
        };

        let tokens = tokenize(&source)?;
        let nodes = parse(tokens)?;

        let module = ast::Module::new(nodes);
        modules.insert(relative_path, module);
    }

    return Ok(modules);
}

pub struct Symbol {}
pub struct SymbolTable {}
