use super::nodes::{ast, hlir};
use super::FILE_EXTENSION;
use super::{errors::CompileCtx, parser::ParsedFile};

mod infere_type;
mod types;

mod nodes;

mod expression;
mod variables;
use types::ModuleTypes;
pub use types::ParsedProject;
pub use variables::{Variable, Variables};

#[derive(Debug, Default)]
pub struct AnalyzedModule {
    pub functions: Vec<hlir::Function>,
}

pub fn analyze(ctx: &mut CompileCtx, mut project: ParsedProject) -> AnalyzedModule {
    let types = types::parse_types(ctx, &mut project);

    let mut analyzed_module = AnalyzedModule::default();

    for file in vec![project.main, project.std] {
        handle_file(ctx, &types, &mut analyzed_module, file);
    }

    return analyzed_module;
}

fn handle_file(
    ctx: &mut CompileCtx,
    types: &ModuleTypes,
    analyzed_module: &mut AnalyzedModule,
    file: ParsedFile,
) {
    ctx.set_current_path(file.relative_file_path);
    ctx.set_status(format!(
        "Analyzing: {}.{FILE_EXTENSION}",
        ctx.current_file_path.into_path_buf().to_string_lossy()
    ));

    for function in file.functions {
        handle_function(ctx, types, analyzed_module, function);
    }

    for (_, file) in file.imports {
        handle_file(ctx, types, analyzed_module, file);
    }
}

fn handle_function(
    ctx: &mut CompileCtx,
    types: &ModuleTypes,
    analyzed_module: &mut AnalyzedModule,
    mut function: ast::Function,
) {
    let mut hlir_function = hlir::Function {
        key: function.raw.key,
        body: Vec::new(),
        return_type: function.raw.return_type.raw.convert(),
        parameters: Vec::new(),
        variables: Variables::new(),
    };
    hlir_function.variables.push_scope();

    for parameter in function.raw.parameters {
        let data_type = parameter.raw.data_type.raw.convert();

        let key = match hlir_function.variables.insert(
            parameter.raw.name.raw,
            parameter.raw.mutable,
            data_type.clone(),
            parameter.location,
        ) {
            Ok(k) => k,
            Err(old) => todo!(),
        };

        hlir_function.parameters.push(hlir::Parameter {
            name: key,
            mutable: parameter.raw.mutable,
            data_type,
        });
    }

    let mut nodes = function.raw.body.drain(..).collect::<Vec<ast::Node>>();
    nodes.reverse();

    let returned = loop {
        let node = match nodes.pop() {
            Some(n) => n,
            None => break false,
        };

        let mut returned = false;
        let node: hlir::Node = match node.raw {
            ast::RawNode::Return(expression) => {
                returned = true;
                hlir_function.handle_return(
                    ctx,
                    types,
                    expression,
                    Some(function.raw.return_type.clone()),
                )
            }
            ast::RawNode::DeclareVariable {
                name,
                mutable,
                data_type,
                expression,
            } => hlir_function.handle_decl(
                ctx,
                types,
                node.location,
                name,
                mutable,
                data_type,
                expression,
            ),
            ast::RawNode::SetPath(path, expression) => {
                hlir_function.handle_set(ctx, types, node.location, path, expression)
            }

            raw => {
                ctx.error(node.location, format!("Not yet implemented: {raw:#?}"));
                continue;
            }
        };

        hlir_function.body.push(node);
        if returned {
            break returned;
        }
    };

    if !returned {
        if matches!(hlir_function.return_type, hlir::Type::Void) {
            hlir_function
                .body
                .push(hlir::Node::Return(hlir::Type::Void, None));
        } else {
            ctx.error(function.location, "Missing return");
        }
    }

    hlir_function.variables.pop_scope();

    for (_, variable) in &hlir_function.variables.map {
        let name = &variable.name;

        if !variable.used {
            ctx.warning(
                variable.location.clone(),
                format!("The value of '{name}' is assigned but never used!"),
            );
            continue;
        }
        if variable.mutable && !variable.modified {
            ctx.warning(
                variable.location.clone(),
                format!("The value of '{name}' is never modified!"),
            );
            continue;
        }
    }

    analyzed_module.functions.push(hlir_function);
}
