use crate::compiler::{
    analyzer::{analyzer::what_type, FunctionCtx, IRType, IRValue, ProgramCtx},
    errors::Location,
    parser::{Expression, ExpressionInfo, Value},
    types::{ReferenceState, Type},
};

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

            let value = handle_read(program, function, location, data_type, *index);

            function.operations.getelementptr_inbounds(
                &result_ptr,
                &IRType::Array(size, Box::new(inner_type.clone().convert())),
                &array.key,
                &value,
            );

            function.operations.load(
                &result,
                &inner_type.convert(),
                &IRValue::Variable(result_ptr),
            );

            IRValue::Variable(result)
        }
        Expression::BinaryOperation(a, operator, b) => {
            let result = function.variables.increment();

            let first = *a;
            let second = *b;

            let first_value = handle_read(program, function, location, &data_type, first);
            let second_value = handle_read(program, function, location, &data_type, second);

            function.operations.binary_operation(
                &result,
                &operator,
                &data_type.convert(),
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
            if !variable.is_pointer_value {
                return IRValue::Variable(variable.key.clone());
            }

            function.operations.load(
                &load_destination,
                &data_type.convert(),
                &IRValue::Variable(variable.key.clone()),
            );

            IRValue::Variable(load_destination)
        }
        _ => todo!("{info:#?}"),
    };
}
