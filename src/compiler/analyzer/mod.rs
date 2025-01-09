use std::collections::VecDeque;

use super::nodes::{ast, hlir};
use super::{errors::CompileCtx, parser::ParsedFile};

mod infere_type;
mod types;

mod variable;
mod expression;
mod result;

mod variables;
pub use variables::Variables;

#[derive(Default)]
pub struct AnalyzedModule {
    pub functions: Vec<hlir::Function>,
}

pub fn analyze(ctx: &mut CompileCtx, mut files: Vec<ParsedFile>) -> AnalyzedModule {
    types::parse_types(ctx, &mut files);

    let mut analyzed_module = AnalyzedModule::default();

    for file in files {
        handle_file(ctx, &mut analyzed_module, file);
    }

    return analyzed_module;
}

fn handle_file(ctx: &mut CompileCtx, analyzed_module: &mut AnalyzedModule, file: ParsedFile) {
    for (_, file) in file.imports {
        handle_file(ctx, analyzed_module, file);
    }

    for function in file.functions {
        handle_function(ctx, analyzed_module, function);
    }
}

fn handle_function(
    ctx: &mut CompileCtx,
    analyzed_module: &mut AnalyzedModule,
    mut function: ast::Function,
) {
    let mut mir_function = hlir::Function {
        key: function.raw.key,
        body: Vec::new(),
        return_type: function.raw.return_type.raw.convert(),
        parameters: Vec::new(),
        variables: Variables::new()
    };

    let mut nodes = function.raw.body.drain(..).collect::<VecDeque<ast::Node>>();
    let return_type = Some(function.raw.return_type);
    
    let returned = loop {
        let node = match nodes.pop_front() {
            Some(n) => n,
            None => break false,
        };

        let mut returned = false;
        let node: hlir::Node = match node.raw {
            ast::RawNode::Return(expression) => {
                returned = true;
                mir_function.handle_return(ctx, expression, &return_type)
            }
            ast::RawNode::DeclareVariable {
                name,
                mutable,
                data_type,
                expression,
            } => mir_function.handle_decl(ctx, node.location, name, mutable, data_type, expression),
            raw => {
                ctx.error(node.location, format!("Not yet implemented: {raw:#?}"));
                continue;
            }
        };

        mir_function.body.push(node);
        if returned {
            break returned;
        }
    };

    if !returned {
        if matches!(mir_function.return_type, hlir::Type::Void) {
            mir_function
                .body
                .push(hlir::Node::Return(hlir::Type::Void, None));
        } else {
            ctx.error(function.location, "Missing return");
        }
    }

    let _ = analyzed_module.functions.push(mir_function);
}

