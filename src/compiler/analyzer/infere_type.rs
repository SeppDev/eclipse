use crate::compiler::{
    errors::CompileCtx,
    nodes::{ast, hlir},
};

use super::{expression::InvokeType, types::ModuleTypes};

impl hlir::Function {
    pub(super) fn infere_type(
        &self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        declared_type: Option<ast::Type>,
        expression: &ast::Expression,
    ) -> hlir::Type {
        if let Some(declared_type) = declared_type {
            let converted = declared_type.raw.convert();
            let infered = self.infere(ctx, types, Some(&declared_type.raw), expression);

            if converted != infered {
                ctx.error(
                    declared_type.location.clone(),
                    format!("Expected {converted} but got {infered}"),
                );
            }

            infered
        } else {
            self.infere(ctx, types, None, expression)
        }
    }

    fn infere(
        &self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        expected_type: Option<&ast::RawType>,
        expression: &ast::Expression,
    ) -> hlir::Type {
        return match &expression.raw {
            ast::RawExpression::Group(expression) => {
                self.infere(ctx, types, expected_type, expression)
            }
            ast::RawExpression::Increment(expression) => {
                let data_type = self.infere(ctx, types, expected_type, expression);
                if !data_type.is_integer() {
                    ctx.error(expression.location.clone(), "Can only increment integers");
                    return hlir::Type::Int(ctx.target.pointer_width());
                }
                data_type
            }
            ast::RawExpression::Decrement(expression) => {
                let data_type = self.infere(ctx, types, expected_type, expression);
                if !data_type.is_integer() {
                    ctx.error(expression.location.clone(), "Can only decrement integers");
                    return hlir::Type::Int(ctx.target.pointer_width());
                }
                data_type
            }
            ast::RawExpression::Tuple(expressions) => {
                let types = match expected_type {
                    Some(expected) => {
                        if let ast::RawType::Tuple(tuple) = &expected {
                            tuple
                                .iter()
                                .zip(expressions)
                                .map(|(data_type, expression)| {
                                    self.infere(ctx, types, Some(&data_type.raw), expression)
                                })
                                .collect()
                        } else {
                            Vec::new()
                        }
                    }
                    None => expressions
                        .iter()
                        .map(|expression| self.infere(ctx, types, expected_type, expression))
                        .collect::<Vec<hlir::Type>>(),
                };

                hlir::Type::Tuple(types)
            }
            ast::RawExpression::Invoke(path, _) => {
                let invoke_type = self.get_invoke_type(ctx, types, path).raw;
                match invoke_type {
                    InvokeType::Unkown => hlir::Type::default(),
                    InvokeType::Function { return_type, .. } => return_type.clone(),
                }
            }
            ast::RawExpression::GetPath(path) => {
                if path.raw.len() == 1 {
                    let name = path.raw.first().unwrap();
                    match self.variables.get(name) {
                        Some(var) => return var.data_type.clone(),
                        None => {}
                    };
                }
                hlir::Type::default()
            }
            ast::RawExpression::Integer(_) => {
                if let Some(ast) = expected_type {
                    let converted = ast.convert();
                    if converted.is_integer() {
                        converted
                    } else {
                        hlir::Type::Int(ctx.target.pointer_width())
                    }
                } else {
                    hlir::Type::Int(ctx.target.pointer_width())
                }
            }
            ast::RawExpression::Boolean(_) => hlir::Type::Boolean,
            ast::RawExpression::ArithmeticOperation(first, _, second) => {
                let first = self.infere(ctx, types, expected_type, &first);
                let second = self.infere(ctx, types, expected_type, &second);

                if first != second {
                    ctx.error(
                        expression.location.clone(),
                        format!("Expected {first} but got {second}"),
                    );
                }
                first
            }
            raw => {
                ctx.error(
                    expression.location.clone(),
                    format!("Not yet implemented: {raw:#?}"),
                );
                hlir::Type::default()
            }
        };
    }
}

impl hlir::Type {
    fn is_integer(&self) -> bool {
        use hlir::Type::*;
        matches!(self, UInt(..) | Int(..) | Isize | Usize)
    }
}

impl ast::RawType {
    pub(super) fn convert(&self) -> hlir::Type {
        match self {
            Self::Void => hlir::Type::Void,
            Self::Tuple(fields) if fields.len() == 0 => hlir::Type::Void,
            Self::Tuple(fields) if fields.len() == 1 => fields.first().unwrap().raw.convert(),
            Self::UInt(n) => hlir::Type::UInt(*n),
            Self::Int(n) => hlir::Type::Int(*n),
            Self::Boolean => hlir::Type::Boolean,
            Self::Isize => hlir::Type::Isize,
            Self::Usize => hlir::Type::Usize,
            Self::Tuple(types) => {
                hlir::Type::Tuple(types.iter().map(|t| t.raw.convert()).collect())
            }
            _ => todo!(),
        }
    }
}
