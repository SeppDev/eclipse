use std::collections::{HashMap, VecDeque};

use infere_type::infere_type;

use super::nodes::ast::{Identifier, Located};
use super::nodes::{ast, hlir};
use super::{errors::CompileCtx, parser::ParsedFile};

mod infere_type;
mod types;

#[derive(Debug, Default)]
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
    let return_type = match &function.raw.return_type {
        Some(r) => r.raw.convert(),
        None => hlir::Type::Void,
    };

    let mut mir_function = hlir::Function {
        key: function.raw.key,
        body: Vec::new(),
        return_type,
        parameters: Vec::new(),
    };

    let mut nodes = function.raw.body.drain(..).collect::<VecDeque<ast::Node>>();

    let returned = loop {
        let node = match nodes.pop_front() {
            Some(n) => n,
            None => break false,
        };

        let mut returned = false;
        let node: hlir::Node = match node.raw {
            ast::RawNode::Return(expression) => {
                returned = true;
                handle_return(ctx, expression, &function.raw.return_type)
            }
            ast::RawNode::DeclareVariable {
                name,
                mutable,
                data_type,
                expression,
            } => handle_decl(ctx, mutable, name, data_type, expression),
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
                .push(hlir::Node::Return(hlir::Type::Void, None))
        } else {
        }
    }

    let _ = analyzed_module.functions.push(mir_function);
}

fn handle_decl(
    ctx: &mut CompileCtx,
    mutable: Option<Located<bool>>,
    name: Identifier,
    data_type: Option<ast::Type>,
    expression: Option<ast::Expression>,
) -> hlir::Node {
    let expression = match expression {
        Some(expression) => expression,
        None => todo!(),
    };

    let data_type = infere_type(ctx, &data_type, &expression);
    let expression = handle_expression(ctx, expression, data_type.clone());

    hlir::Node::DeclareVariable {
        name: name.raw,
        mutable: mutable.is_some(),
        data_type,
        expression,
    }
}

fn handle_return(
    ctx: &mut CompileCtx,
    expression: Option<ast::Expression>,
    data_type: &Option<ast::Type>,
) -> hlir::Node {
    match expression {
        Some(expression) => {
            let data_type = infere_type(ctx, &data_type, &expression);
            let expression = handle_expression(ctx, expression, data_type.clone());
            hlir::Node::Return(data_type, Some(expression))
        }
        None => {
            let data_type = match data_type {
                Some(dt) => dt,
                None => return hlir::Node::Return(hlir::Type::Void, None),
            };
            
            let converted = data_type.raw.convert();
            if !matches!(converted, hlir::Type::Void) {
                ctx.error(data_type.location.clone(), "Expected return");
            }
            
            hlir::Node::Return(converted, None)
        }
    }
}

fn handle_expression(
    ctx: &mut CompileCtx,
    expression: ast::Expression,
    data_type: hlir::Type,
) -> hlir::Expression {
    let raw = match expression.raw {
        ast::RawExpression::Integer(value) => hlir::RawExpression::Integer(value),
        ast::RawExpression::Boolean(value) => hlir::RawExpression::Boolean(value),
        raw => {
            ctx.error(
                expression.location,
                format!("Not yet implemented: {raw:#?}"),
            );
            hlir::RawExpression::default()
        }
    };
    hlir::Expression::new(raw, data_type)
}
