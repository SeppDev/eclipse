use std::collections::{HashMap, VecDeque};

use infere_type::infere_type;

use super::nodes::ast::{Identifier, Located};
use super::nodes::{ast, mir};
use super::{errors::CompileCtx, parser::ParsedFile};

mod infere_type;
mod types;

#[derive(Debug, Default)]
pub struct AnalyzedModule {
    pub functions: Vec<mir::Function>,
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
        None => mir::Type::Void,
    };

    let mut mir_function = mir::Function {
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
        let node: mir::Node = match node.raw {
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
        if matches!(mir_function.return_type, mir::Type::Void) {
            mir_function
                .body
                .push(mir::Node::Return(mir::Type::Void, None))
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
) -> mir::Node {
    let expression = match expression {
        Some(expression) => expression,
        None => todo!(),
    };

    let data_type = infere_type(ctx, &data_type, &expression);
    let expression = handle_expression(ctx, expression, data_type.clone());

    mir::Node::DeclareVariable {
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
) -> mir::Node {
    match expression {
        Some(expression) => {
            let data_type = infere_type(ctx, &data_type, &expression);
            let expression = handle_expression(ctx, expression, data_type.clone());
            mir::Node::Return(data_type, Some(expression))
        }
        None => {
            let data_type = match data_type {
                Some(dt) => dt,
                None => return mir::Node::Return(mir::Type::Void, None),
            };
            
            let converted = data_type.raw.convert();
            if !matches!(converted, mir::Type::Void) {
                ctx.error(data_type.location.clone(), "Expected return");
            }
            
            mir::Node::Return(converted, None)
        }
    }
}

fn handle_expression(
    ctx: &mut CompileCtx,
    expression: ast::Expression,
    data_type: mir::Type,
) -> mir::Expression {
    let raw = match expression.raw {
        ast::RawExpression::Integer(value) => mir::RawExpression::Integer(value),
        ast::RawExpression::Boolean(value) => mir::RawExpression::Boolean(value),
        raw => {
            ctx.error(
                expression.location,
                format!("Not yet implemented: {raw:#?}"),
            );
            mir::RawExpression::default()
        }
    };
    mir::Expression::new(raw, data_type)
}
