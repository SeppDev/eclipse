use crate::compiler::{
    errors::CompileCtx,
    nodes::{ast, hlir},
};

pub fn infere_type(
    ctx: &mut CompileCtx,
    expected_type: &Option<ast::Type>,
    expression: &ast::Expression,
) -> hlir::Type {
    let infered = infere(ctx, &expected_type, expression);

    if let Some(expected) = expected_type {
        let expected_type: hlir::Type = expected.raw.convert();
        if expected_type != infered {
            ctx.error(
            expected.location.clone(),
            format!("Expected {expected_type} but got {infered}"),
            );
        }
    }

    infered
}

fn infere(
    ctx: &mut CompileCtx,
    expected_type: &Option<ast::Type>,
    expression: &ast::Expression,
) -> hlir::Type {
    return match &expression.raw {
        &ast::RawExpression::Integer(_) => {
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
        &ast::RawExpression::Boolean(_) => {
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
