use crate::compiler::{errors::CompileCtx, nodes::{ast::{self, Identifier, Located}, hlir}};

use super::infere_type::infere_type;

impl hlir::Function {
    pub fn handle_decl(
        &mut self,
        ctx: &mut CompileCtx,
        name: Identifier,
        mutable: Option<Located<bool>>,
        data_type: Option<ast::Type>,
        expression: Option<ast::Expression>,
    ) -> hlir::Node {
        let data_type = infere_type(ctx, &data_type, &expression);

        let expression = match expression {
            Some(expression) => expression,
            None => todo!(),
        };

        let expression = self.handle_expression(ctx, expression, data_type.clone());

        hlir::Node::DeclareVariable {
            name: name.raw,
            mutable: mutable.is_some(),
            data_type,
            expression,
        }
    }
}
