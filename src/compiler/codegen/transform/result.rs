use crate::compiler::{
    errors::CompileCtx,
    nodes::{hlir, ir},
};

impl ir::Function {
    pub(super) fn handle_return(
        &mut self,
        ctx: &mut CompileCtx,
        data_type: hlir::Type,
        expression: Option<hlir::Expression>,
    ) {
        let expression = match expression {
            Some(expr) => expr,
            None => {
                return self.push(ir::Instruction::Return(
                    ctx.target.convert(&data_type),
                    None,
                ));
            }
        };

        let value: ir::Value = match expression.raw {
            hlir::RawExpression::Integer(value) => ir::Value::Integer(value),
            _ => todo!(),
        };

        self.push(ir::Instruction::Return(
            ctx.target.convert(&data_type),
            Some(value),
        ));
    }
}
