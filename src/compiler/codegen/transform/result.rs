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
        let ir_type = ctx.target.convert(&data_type);

        let expression = match expression {
            Some(expr) => expr,
            None => {
                self.r#return(&ir_type, None);
                return
            }
        };

        let value = self.handle_expression(ctx, expression);
        self.r#return(&ir_type, Some(&value));
    }
}
