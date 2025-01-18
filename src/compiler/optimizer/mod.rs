use std::rc::Rc;

use super::{
    analyzer::AnalyzedModule,
    errors::CompileCtx,
    nodes::{ast, hlir},
};

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

        self.variables.map.iter_mut().for_each(|(_, variable)| {
            variable.mutable = variable.modified;
        });

        let function = Rc::new(&mut self);

        let new_body = body
            .into_iter()
            .map(|node| node.optimize_node())
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
    fn optimize_node(self) -> Self {
        return match self {
            hlir::Node::DeclareVariable {
                name,
                data_type,
                expression,
            } => hlir::Node::DeclareVariable {
                name,
                data_type,
                expression: expression.optimize_expression(),
            },
            hlir::Node::Return(data_type, expression) => hlir::Node::Return(
                data_type,
                match expression {
                    Some(expression) => Some(expression.optimize_expression()),
                    None => None,
                },
            ),
            _ => self,
        };
    }
}

impl hlir::Expression {
    fn optimize_expression(self) -> Self {
        match self.raw {
            hlir::RawExpression::BinaryOperation(first, operation, second) => {
                let first = first.optimize_expression();
                let second = second.optimize_expression();

                if let (hlir::RawExpression::Integer(first), hlir::RawExpression::Integer(second)) =
                    (&first.raw, &second.raw)
                {
                    let first = first.parse::<isize>().expect("Failed to parse integer");
                    let second = second.parse::<isize>().expect("Failed to parse integer");

                    let result = match operation {
                        ast::ArithmeticOperator::Add => first + second,
                        ast::ArithmeticOperator::Subtract => first - second,
                        ast::ArithmeticOperator::Multiply => first * second,
                        ast::ArithmeticOperator::Remainder => first % second,
                        ast::ArithmeticOperator::Divide => first / second,
                    };

                    return Self {
                        data_type: self.data_type,
                        raw: hlir::RawExpression::Integer(format!("{result}")),
                    };
                }
                return Self {
                    data_type: self.data_type,
                    raw: hlir::RawExpression::BinaryOperation(
                        Box::new(first),
                        operation,
                        Box::new(second),
                    ),
                };
            }
            // hlir::RawExpression::Group(expression) => {
            // hlir::RawExpression::Group(Box::new(expression.optimize_expression()))
            // }
            _ => return self,
        };
    }
}
