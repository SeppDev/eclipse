pub use read::handle_read;
use store::handle_array_store;
pub use store::handle_store;

use crate::compiler::{
    analyzer::{analyzer::what_type, FunctionCtx, IRType, IRValue, ProgramCtx},
    errors::Location,
    parser::{ArithmeticOperator, Expression, ExpressionInfo, Value},
    types::{BaseType, ReferenceState, Type},
};

mod read;
mod store;

pub use read::*;
pub use store::*;

use super::handle_call;

pub fn handle_expression(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: &String,
    allocate: bool,
    data_type: &Type,
    info: ExpressionInfo,
) -> IRValue {
    if allocate {
        let should_allocate = !matches!(info.expression, Expression::Value(_));
        if should_allocate {
            function
                .operations
                .allocate(&destination, &data_type.convert())
        }
    }

    return match info.expression {
        Expression::Value(value) => match value {
            Value::Integer(int) => IRValue::IntLiteral(int),
            Value::Boolean(bool) => IRValue::BoolLiteral(bool),
            Value::Float(float) => IRValue::FloatLiteral(float),
            _ => todo!(),
        },
        Expression::Minus(expression) => {
            let value = handle_read(program, function, location, &data_type, *expression);

            function.operations.binary_operation(
                &destination,
                &ArithmeticOperator::Subtract,
                &data_type.convert(),
                &IRValue::IntLiteral(String::from("0")),
                &value,
            );

            IRValue::Variable(destination.clone())
        }
        Expression::Not(expression) => {
            let value = handle_read(program, function, location, &data_type, *expression);

            function.operations.xor_boolean(&destination, &value);

            IRValue::Variable(destination.clone())
        }
        Expression::Index(name, index) => {
            let result_ptr = function.variables.increment();

            let array = match function.variables.read(&name, &ReferenceState::None) {
                Some(var) => var.clone(),
                None => {
                    program.debug.error(
                        info.location.clone(),
                        format!("Could not find variable named: '{name}'"),
                    );
                    return IRValue::Null;
                }
            };

            let (inner_type, size) = array.data_type.array_info();

            let index_type = Type::new(BaseType::Int(32));
            let value = handle_read(program, function, location, &index_type, *index);

            function.operations.getelementptr_inbounds(
                &result_ptr,
                &IRType::Array(size, Box::new(inner_type.convert())),
                &array.key,
                &index_type.convert(),
                &value,
            );

            function.operations.load(
                &destination,
                &data_type.convert(),
                &IRValue::Variable(result_ptr),
            );

            IRValue::Variable(destination.clone())
        }
        Expression::BinaryOperation(first, operator, second) => {
            let first_value = handle_read(program, function, location, &data_type, *first);
            let second_value = handle_read(program, function, location, &data_type, *second);

            function.operations.binary_operation(
                &destination,
                &operator,
                &data_type.convert(),
                &first_value,
                &second_value,
            );

            IRValue::Variable(destination.clone())
        }
        Expression::CompareOperation(a, operator, b) => {
            let value_type = what_type(program, function, location, None, &*a);
            let first_value = handle_read(program, function, location, &value_type, *a);
            let second_value = handle_read(program, function, location, &value_type, *b);

            function.operations.compare_operation(
                &destination,
                &operator,
                &value_type.convert(),
                &first_value,
                &second_value,
            );
            IRValue::Variable(destination.clone())
        }
        Expression::GetVariable(name) => {
            let variable = match function.variables.read(&name, &ReferenceState::None) {
                Some(var) => var,
                None => {
                    program.debug.error(
                        location.clone(),
                        format!("Could not find variable named: '{name}'"),
                    );
                    return IRValue::Null;
                }
            };
            if !variable.is_pointer_value || !variable.data_type.base.is_basic() {
                return IRValue::Variable(variable.key.clone());
            }

            if variable.data_type.base.is_basic() {
                function.operations.load(
                    &destination,
                    &data_type.convert(),
                    &IRValue::Variable(variable.key.clone()),
                );
            } else {
                function.operations.memcpy(
                    &destination,
                    &variable.key,
                    &variable.data_type.base.bytes(),
                    false,
                )
            }

            IRValue::Variable(destination.clone())
        }
        Expression::Array(items) => {
            handle_array_store(
                program,
                function,
                location,
                items,
                &destination,
                data_type,
                0,
            );
            IRValue::Variable(destination.clone())
        }
        Expression::Call(path, arguments) => {
            handle_call(
                program,
                function,
                Some((&destination, data_type)),
                location,
                path,
                arguments,
            );
            IRValue::Variable(destination.clone())
        }
        _ => todo!("{info:#?}"),
    };
}
