use crate::compiler::{
    errors::CompileCtx,
    nodes::{
        ast::{self, Expression},
        hlir,
    },
};

use super::types::ModuleTypes;

impl hlir::Function {
    pub(super) fn handle_expression(
        &mut self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        expression: ast::Expression,
        data_type: hlir::Type,
    ) -> hlir::Expression {
        let raw = match expression.raw {
            ast::RawExpression::Invoke(path, arguments) => {
                let invoke_type = self.get_invoke_type(ctx, types, &path);
                self.handle_invoke(ctx, types, invoke_type, arguments)
            }
            ast::RawExpression::GetPath(mut path) => {
                let name = path.raw.pop().unwrap();
                match self.variables.read(&name) {
                    Some(variable) => hlir::RawExpression::GetVariable(variable.key.clone()),
                    None => {
                        ctx.error(
                            expression.location,
                            format!("Cannot find value '{name}' in this scope"),
                        );
                        hlir::RawExpression::default()
                    }
                }
            }
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

    fn handle_invoke(
        &mut self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        invoke_type: InvokeType,
        arguments: Vec<ast::Expression>,
    ) -> hlir::RawExpression {
        match invoke_type {
            InvokeType::Unkown => hlir::RawExpression::default(),
            InvokeType::Function {
                key,
                parameters,
                return_type,
            } => hlir::RawExpression::Call(key.clone(), Vec::new()),
        }
    }
    pub(super) fn get_invoke_type<'a>(
        &self,
        ctx: &mut CompileCtx,
        types: &'a ModuleTypes,
        path: &Box<Expression>,
    ) -> InvokeType<'a> {
        match &path.raw {
            ast::RawExpression::GetPath(path) => {
                let function = match types.get_path(ctx, &path.raw) {
                    Some(f) => f,
                    None => {
                        ctx.error(path.location.clone(), format!("Could not find: {}", path.raw));
                        return InvokeType::default();
                    }
                };
                InvokeType::Function {
                    key: &function.key,
                    parameters: &function.parameters,
                    return_type: &function.return_type,
                }
            }
            raw => {
                ctx.error(
                    path.location.clone(),
                    format!("Path expression not yet implemented: {raw:#?}"),
                );
                InvokeType::default()
            }
        }
    }
}

#[derive(Default)]
pub(super) enum InvokeType<'a> {
    #[default]
    Unkown,
    Function {
        key: &'a String,
        parameters: &'a Vec<hlir::Type>,
        return_type: &'a hlir::Type,
    },
}
