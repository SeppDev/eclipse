use crate::compiler::{errors::CompileCtx, nodes::{ast, hlir}};

impl hlir::Function {
    pub fn handle_expression(
        &mut self,
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

}