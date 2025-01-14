use crate::compiler::{
    errors::CompileCtx,
    nodes::{
        ast::{self, Expression, Located},
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
                    Some(variable) => {
                        variable.used = true;
                        hlir::RawExpression::GetVariable(variable.key.clone())
                    },
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
            ast::RawExpression::BinaryOperation(first, operator, second) => {
                let first = self.handle_expression(ctx, types, *first, data_type.clone());
                let second = self.handle_expression(ctx, types, *second, data_type.clone());
                
                hlir::RawExpression::BinaryOperation(
                    Box::new(first),
                    operator,
                    Box::new(second),
                )   
            }
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
        invoke: Invoke,
        mut arguments: Vec<ast::Expression>,
    ) -> hlir::RawExpression {
        match invoke.raw {
            InvokeType::Unkown => hlir::RawExpression::default(),
            InvokeType::Function {
                key,
                parameters,
                return_type,
            } => {
                let mut ir_arguments = Vec::new();

                if arguments.len() != parameters.len() {
                    ctx.error(
                        invoke.location,
                        format!(
                            "Expected {} arguments, found {}",
                            parameters.len(),
                            arguments.len()
                        ),
                    );
                    return hlir::RawExpression::default();
                }
                arguments.reverse();

                for parameter in parameters.into_iter() {
                    let argument = arguments.pop().unwrap();

                    let expression = self.handle_expression(ctx, types, argument, parameter.clone());
                    ir_arguments.push(expression);
                }

                hlir::RawExpression::Call(
                    key.clone(),
                    ir_arguments,
                )
            }
        }
    }
    pub(super) fn get_invoke_type<'a>(
        &self,
        ctx: &mut CompileCtx,
        types: &'a ModuleTypes,
        path: &Box<Expression>,
    ) -> Invoke<'a> {
        let t = match &path.raw {
            ast::RawExpression::GetPath(path) => {
                match types.get_path(ctx, &path.raw) {
                    Some(function) => {
                        InvokeType::Function {
                            key: &function.key,
                            parameters: &function.parameters,
                            return_type: &function.return_type,
                        }
                    },
                    None => {
                        ctx.error(
                            path.location.clone(),
                            format!("Could not find: {}", path.raw),
                        );
                        InvokeType::default()
                    }
                }
            }
            raw => {
                ctx.error(
                    path.location.clone(),
                    format!("Path expression not yet implemented: {raw:#?}"),
                );
                InvokeType::default()
            }
        };
        return Located::new(path.location.clone(), t);
    }
}

pub type Invoke<'a> = Located<InvokeType<'a>>;

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
