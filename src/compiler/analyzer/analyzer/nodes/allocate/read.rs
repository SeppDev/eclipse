use crate::compiler::{
    analyzer::{analyzer::{handle_call, what_type}, FunctionCtx, IRType, IRValue, ProgramCtx},
    errors::Location,
    parser::{ArithmeticOperator, Expression, ExpressionInfo, Value},
    types::{BaseType, ReferenceState, Type},
};

use super::store::handle_array_store;

pub fn handle_read(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    data_type: &Type,
    info: ExpressionInfo,
) -> IRValue {
    return match info.expression {
        Expression::Value(value) => match value {
            Value::Integer(int) => IRValue::IntLiteral(int),
            Value::Boolean(bool) => IRValue::BoolLiteral(bool),
            Value::Float(float) => IRValue::FloatLiteral(float),
            _ => todo!(),
        },
        Expression::Minus(expression) => {
            let result = function.variables.increment();
            let value = handle_read(program, function, location, &data_type, *expression);

            function.operations.binary_operation(
                &result,
                &ArithmeticOperator::Subtract,
                &data_type.convert(),
                &IRValue::IntLiteral(String::from("0")),
                &value,
            );

            IRValue::Variable(result)
        }
        Expression::Not(expression) => {
            let result = function.variables.increment();
            let value = handle_read(program, function, location, &data_type, *expression);

            function.operations.xor_boolean(&result, &value);

            IRValue::Variable(result)
        }
        Expression::Index(name, index) => {
            let result_ptr = function.variables.increment();
            let result = function.variables.increment();

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
                &value,
            );

            function.operations.load(
                &result,
                &data_type.convert(),
                &IRValue::Variable(result_ptr),
            );

            IRValue::Variable(result)
        }
        Expression::BinaryOperation(first, operator, second) => {
            let result = function.variables.increment();

            let first_value = handle_read(program, function, location, &data_type, *first);
            let second_value = handle_read(program, function, location, &data_type, *second);

            function.operations.binary_operation(
                &result,
                &operator,
                &data_type.convert(),
                &first_value,
                &second_value,
            );
            IRValue::Variable(result)
        }
        Expression::CompareOperation(a, operator, b) => {
            let result = function.variables.increment();

            let first = *a;
            let second = *b;

            let value_type = what_type(program, function, location, None, &first);
            let first_value = handle_read(program, function, location, &value_type, first);
            let second_value = handle_read(program, function, location, &value_type, second);

            function.operations.compare_operation(
                &result,
                &operator,
                &value_type.convert(),
                &first_value,
                &second_value,
            );
            IRValue::Variable(result)
        }
        Expression::GetVariable(name) => {
            let load_destination = function.variables.increment();

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

            function.operations.load(
                &load_destination,
                &data_type.convert(),
                &IRValue::Variable(variable.key.clone()),
            );

            IRValue::Variable(load_destination)
        }
        Expression::Array(items) => {
            let result = function.variables.increment();
            handle_array_store(program, function, location, items, &result, data_type, 0);
            IRValue::Variable(result)
        }
        _ => todo!("{info:#?}"),
    };
}
