use std::rc::Rc;

use super::{analyzer::AnalyzedModule, errors::CompileCtx, nodes::hlir};

pub fn optimize(ctx: &mut CompileCtx, mut module: AnalyzedModule) -> AnalyzedModule {
    module.functions = module
        .functions
        .into_iter()
        .map(|f| f.check_function())
        .collect();

    module
}

impl hlir::Function {
    fn check_function(mut self) -> Self {
        let body = self.body.drain(..).collect::<Vec<hlir::Node>>();

        let function = Rc::new(&mut self);

        let new_body = body
            .into_iter()
            .filter_map(|node| node.is_essential(&function.clone()).then_some(node))
            .collect::<Vec<hlir::Node>>();

        self.body = new_body;
        self
    }
}

impl hlir::Node {
    fn is_essential(&self, function: &Rc<&mut hlir::Function>) -> bool {
        self.is_essential_variable(function)
    }
    fn is_essential_variable(&self, function: &Rc<&mut hlir::Function>) -> bool {
        let (key, expression) = if let hlir::Node::DeclareVariable {
            name, expression, ..
        } = self
        {
            (name, expression)
        } else {
            return true;
        };

        if !matches!(
            expression.raw,
            hlir::RawExpression::GetVariable(..)
                | hlir::RawExpression::Integer(..)
                | hlir::RawExpression::Boolean(..)
                | hlir::RawExpression::Float(..)
        ) {
            return true;
        }
        
        let variable = function.variables.map.get(key).unwrap();
        return variable.used;
    }
}
