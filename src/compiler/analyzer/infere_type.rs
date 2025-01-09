use crate::compiler::{
    errors::CompileCtx,
    nodes::{ast, hlir},
};

impl hlir::Function {
    pub fn infere_type(
        &self,
        ctx: &mut CompileCtx,
        declared_type: &Option<ast::Type>,
        expression: &Option<ast::Expression>,
    ) -> hlir::Type {
        let infered = match expression {
            Some(expr) => self.infere(ctx, &declared_type, expr),
            None => hlir::Type::Void,
        };

        if let Some(declared_type) = declared_type {
            let converted = declared_type.raw.convert();
            if converted != infered {
                ctx.error(
                    declared_type.location.clone(),
                    format!("Expected {converted} but got {infered}"),
                );
            }
        }

        infered
    }

    fn infere(
        &self,
        ctx: &mut CompileCtx,
        expected_type: &Option<ast::Type>,
        expression: &ast::Expression,
    ) -> hlir::Type {
        return match &expression.raw {
            ast::RawExpression::GetPath(path) if path.raw.len() == 1 => {
                let name = path.raw.first().unwrap();
                match self.variables.get(name) {
                    Some(var) => var.data_type.clone(),
                    None => todo!()
                }
            }
            ast::RawExpression::Integer(_) => {
                if let Some(ast) = expected_type {
                    if ast.raw.is_integer() {
                        ast.raw.convert()
                    } else {
                        hlir::Type::Int(32)
                    }
                } else {
                    hlir::Type::Int(32)
                }
            }
            ast::RawExpression::Boolean(_) => {
                if let Some(ast) = expected_type {
                    if ast.raw.is_boolean() {
                        ast.raw.convert()
                    } else {
                        hlir::Type::Boolean
                    }
                } else {
                    hlir::Type::Boolean
                }
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

impl ast::RawType {
    fn is_integer(&self) -> bool {
        use ast::RawType::*;
        matches!(self, UInt(..) | Int(..) | Isize | Usize)
    }
    fn is_boolean(&self) -> bool {
        use ast::RawType::*;
        matches!(self, Boolean)
    }
    pub(super) fn convert(&self) -> hlir::Type {
        match self {
            Self::Void => hlir::Type::Void,
            Self::Tuple(fields) if fields.len() == 0 => hlir::Type::Void,
            Self::Tuple(fields) if fields.len() == 1 => fields.first().unwrap().raw.convert(),
            Self::UInt(n) => hlir::Type::UInt(*n),
            Self::Int(n) => hlir::Type::Int(*n),
            _ => todo!(),
        }
    }
}
