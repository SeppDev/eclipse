use crate::compiler::{errors::CompileCtx, nodes::{ast, hlir}};

impl hlir::Function {
    pub fn handle_return(
        &mut self,
        ctx: &mut CompileCtx,
        expression: Option<ast::Expression>,
        return_type: &Option<ast::Type>,
    ) -> hlir::Node {
        let data_type = self.infere_type(ctx, &return_type, &expression);
        
        match expression {
            Some(expression) => {
                let expression = self.handle_expression(ctx, expression, data_type.clone());
                hlir::Node::Return(data_type, Some(expression))
            }
            None => hlir::Node::Return(data_type, None),
        }
    }

}