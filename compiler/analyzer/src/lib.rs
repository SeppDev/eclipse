use std::path::PathBuf;

use context::CompilerCtx;
use diagnostics::DiagnosticData;
use lexer::tokenize;
use parser::parse;
use syntax::ast;

mod semantics;
mod types;

struct Analyzer<'ctx> {
    pub compiler: &'ctx mut CompilerCtx,
}

pub fn analyze(compiler: &mut CompilerCtx, entry: PathBuf) {
    let collection = parse_modules(compiler, entry.clone());
    let entry = collection.modules.get(&entry).unwrap();

    let analyzer = Analyzer { compiler };

    for (relative_path, module) in collection.modules {
        match analyzer.semantics(&module.nodes) {
            Ok(()) => {}
            Err(data) => analyzer.compiler.diagnostics.insert(&relative_path, data),
        }
    }
}

fn parse_modules(compiler: &mut CompilerCtx, entry: PathBuf) -> ast::ModuleCollection {
    let mut to_parse: Vec<PathBuf> = Vec::new();
    to_parse.push(entry);

    let mut collection = ast::ModuleCollection::new();
    while let Some(relative_path) = to_parse.pop() {
        let source = match compiler.read(&relative_path) {
            Some(s) => s,
            None => {
                compiler.diagnostics.insert(
                    &relative_path,
                    DiagnosticData::error().title(format!("Failed to read: {relative_path:?}")),
                );
                continue;
            }
        };

        let tokens = match tokenize(&source) {
            Ok(t) => t,
            Err(data) => {
                compiler.diagnostics.insert(&relative_path, data);
                continue;
            }
        };

        let nodes = match parse(tokens) {
            Ok(n) => n,
            Err(data) => {
                compiler.diagnostics.insert(&relative_path, data);
                continue;
            }
        };

        let module = ast::Module::new(nodes);
        collection.modules.insert(relative_path, module);
    }

    collection
}

pub struct Symbol {}
pub struct SymbolTable {}
