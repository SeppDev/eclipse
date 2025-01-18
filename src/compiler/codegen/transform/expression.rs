use crate::compiler::{
    errors::CompileCtx,
    nodes::{ast, hlir, ir},
};

impl ir::Function {
    pub(super) fn handle_expression(
        &mut self,
        ctx: &mut CompileCtx,
        expression: hlir::Expression,
    ) -> ir::Value {
        let ir_type = ctx.target.convert(&expression.data_type);

        match expression.raw {
            hlir::RawExpression::Integer(value) => {
                return ir::Value::Integer(value);
            }
            hlir::RawExpression::BinaryOperation(first, operator, second) => {
                let result = self.variables.increment();

                let first = self.handle_expression(ctx, *first);
                let second = self.handle_expression(ctx, *second);

                let prefix = match expression.data_type {
                    hlir::Type::Int(_) | hlir::Type::Isize => ir::BinaryOperationPrefix::Signed,
                    hlir::Type::UInt(_) | hlir::Type::Usize => ir::BinaryOperationPrefix::Unsigned,
                    hlir::Type::Float32 | hlir::Type::Float64 => ir::BinaryOperationPrefix::Float,
                    _ => panic!(),
                };

                let operation = match operator {
                    ast::ArithmeticOperator::Add => ir::BinaryOperation::Add(prefix),
                    ast::ArithmeticOperator::Subtract => ir::BinaryOperation::Subtract(prefix),
                    ast::ArithmeticOperator::Divide => ir::BinaryOperation::Divide(prefix),
                    ast::ArithmeticOperator::Multiply => ir::BinaryOperation::Multiply(prefix),
                    ast::ArithmeticOperator::Remainder => ir::BinaryOperation::Remainder(prefix),
                };

                self.binary_operation(&result, &ir_type, &operation, &first, &second);

                return ir::Value::Register(result);
            }
            hlir::RawExpression::GetVariable(name) => {
                let pointer = self.variables.increment();

                let variable = self.variables.get(&name);
                let key = variable.key.clone();

                if variable.is_register_value {
                    return ir::Value::Register(key);
                } else {
                    self.load(
                        &pointer,
                        &ir_type,
                        &ir::Value::Register(variable.key.clone()),
                    );

                    return ir::Value::Register(pointer);
                }
            }
            hlir::RawExpression::Call(..) => {
                let destination = self.variables.increment();
                self.handle_store_expression(ctx, &destination, expression);
                return ir::Value::Register(destination);
            }
            hlir::RawExpression::Group(expression) => self.handle_expression(ctx, *expression),
            _ => todo!(),
        }
    }
    pub(super) fn handle_store_expression(
        &mut self,
        ctx: &mut CompileCtx,
        destination: &String,
        expression: hlir::Expression,
    ) {
        let ir_type = ctx.target.convert(&expression.data_type);

        match expression.raw {
            hlir::RawExpression::Call(key, arguments) => {
                let mut ir_arguments = Vec::new();

                for argument in arguments {
                    let argument_type = ctx.target.convert(&argument.data_type);

                    let value = self.handle_expression(ctx, argument);
                    ir_arguments.push((argument_type, value));
                }

                self.call(Some(&destination), &ir_type, &key, ir_arguments);
            }
            _ => {
                let value = self.handle_expression(ctx, expression);
                self.store(destination, &ir_type, &value);
            }
        }
    }
}
