use diagnostics::DiagnosticData;
use lexer::tokenize;
use parser::parse;
use std::path::PathBuf;
use syntax::ast;

use context::CompilerCtx;

pub fn resolve_modules(compiler: &mut CompilerCtx, entry: &PathBuf) -> ast::ModuleCollection {
    let mut to_parse: Vec<PathBuf> = Vec::new();
    to_parse.push(entry.clone());

    let mut collection = ast::ModuleCollection::default();
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

        let imports: Vec<PathBuf> = nodes
            .iter()
            .filter_map(|node| {
                let import = match &node.raw {
                    ast::RawNode::Import(i) => &i.raw,
                    _ => return None,
                };

                Some(relative_path.join(import))
            })
            .collect();

        to_parse.extend(imports.clone().into_iter());

        let module = ast::Module { nodes };
        collection.modules.insert(relative_path, module);
    }

    collection
}
