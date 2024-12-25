// pub use array::handle_store;
// pub use read::handle_read;

use crate::compiler::{
    analyzer::{analyzer::what_type, FunctionCtx, IRType, IRValue, ProgramCtx},
    errors::Location,
    parser::{ArithmeticOperator, Expression, ExpressionInfo, Value},
    types::{BaseType, ReferenceState, Type},
};

mod array;
pub use array::handle_array_store;

use super::handle_call;

// pub fn handle_read(
//     program: &mut ProgramCtx,
//     function: &mut FunctionCtx,
//     location: &Location,
//     data_type: &Type,
//     info: ExpressionInfo,
// ) -> IRValue {
//     let destination = function.variables.increment();
//     return handle_expression(program, function, location, Some(&destination), data_type, info);
// }

pub fn handle_expression(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: Option<&String>,
    should_allocate: bool,
    data_type: &Type,
    info: ExpressionInfo,
) -> IRValue {
    let value = match info.expression {
        Expression::Value(value) => match value {
            Value::Integer(int) => IRValue::IntLiteral(int),
            Value::Boolean(bool) => IRValue::BoolLiteral(bool),
            Value::Float(float) => IRValue::FloatLiteral(float),
            _ => todo!(),
        },
        // Expression::Minus(expression) => {
        //     let value = handle_read(program, function, location, &data_type, *expression);

        //     function.operations.binary_operation(
        //         &destination,
        //         &ArithmeticOperator::Subtract,
        //         &data_type.convert(),
        //         &IRValue::IntLiteral(String::from("0")),
        //         &value,
        //     );
        // }
        Expression::Not(expression) => {
            let result = function.variables.increment();
            let value = handle_expression(program, function, location, None, false, &data_type, *expression);
            function.operations.xor_boolean(&result, &value);
            IRValue::Variable(result)
        }
        Expression::Index(name, index) => { 
            
            let value = 1234;
            let ptr = &value;
            
            let result_ptr = function.variables.increment();

            let array = match function.variables.read(&name) {
                Some(var) => var.clone(),
                None => {
                    program.debug.error(
                        info.location.clone(),
                        format!("Could not find variable named: '{name}'"),
                    );
                    return IRValue::Null;
                }
            };

            let (inner_type, _) = array.data_type.array_info();

            let index_type = Type::new(BaseType::Usize);
            let _ = what_type(
                program,
                function,
                &index.location,
                Some(&index_type),
                &*index,
            );

            let value = handle_expression(
                program,
                function,
                location,
                None,
                false,
                &index_type,
                *index,
            );

            function.operations.getelementptr_inbounds(
                &result_ptr,
                &array.data_type.convert(),
                &array.key,
                &index_type.convert(),
                &value,
            );

            if let Some(destination) = destination {
                if data_type.base.is_basic() {
                    if should_allocate {
                        function.operations.load_from_pointer(
                            &destination,
                            &inner_type.convert(),
                            &result_ptr,
                        );
                    } else {
                        let result = function.variables.increment();
                        function.operations.load_from_pointer(
                            &result,
                            &inner_type.convert(),
                            &result_ptr,
                        );

                        function.operations.store_from_pointer(
                            &inner_type.convert(),
                            &result,
                            &destination,
                        );
                    }
                } else {
                    function.operations.memcpy(
                        &result_ptr,
                        &destination,
                        &data_type.bytes(),
                        false,
                    );
                }

                return IRValue::Variable(result_ptr);
            } else {
                todo!()
            }
        }
        Expression::BinaryOperation(first, operator, second) => {
            let first_value =
                handle_expression(program, function, location, None, false, &data_type, *first);
            let second_value = handle_expression(
                program, function, location, None, false, &data_type, *second,
            );

            let result = function.variables.increment();
            function.operations.binary_operation(
                &result,
                &operator,
                &data_type.convert(),
                &first_value,
                &second_value,
            );
            IRValue::Variable(result)
        }
        Expression::CompareOperation(first, operator, second) => {
            let value_type = what_type(program, function, location, None, &*first);

            let first_value = handle_expression(
                program,
                function,
                location,
                None,
                false,
                &value_type,
                *first,
            );
            let second_value = handle_expression(
                program,
                function,
                location,
                None,
                false,
                &value_type,
                *second,
            );

            let result = function.variables.increment();
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
            let result = function.variables.increment();

            let variable = match function.variables.read(&name) {
                Some(var) => var,
                None => {
                    program.debug.error(
                        location.clone(),
                        format!("Could not find variable named: '{name}'"),
                    );
                    return IRValue::Null;
                }
            };

            if variable.is_pointer_value {
                if let Some(destination) = destination {
                    if !variable.data_type.base.is_basic() {
                        function.operations.memcpy(
                            &variable.key,
                            destination,
                            &variable.data_type.bytes(),
                            false,
                        );
                        return IRValue::Null;
                    }
                }
                function
                    .operations
                    .load_from_pointer(&result, &data_type.convert(), &variable.key);
                IRValue::Variable(result)
            } else {
                IRValue::Variable(variable.key.clone())
            }
        }
        Expression::Array(items) => {
            if let Some(destination) = destination {
                if should_allocate {
                    function
                        .operations
                        .allocate(&destination, &data_type.convert())
                }
                handle_array_store(
                    program,
                    function,
                    location,
                    items,
                    &destination,
                    data_type,
                    0,
                );
                return IRValue::Null;
            } else {
                todo!()
            }
        }
        Expression::Call(path, arguments) => {
            if let Some(destination) = destination {
                if data_type.base.is_basic() {
                    let result = function.variables.increment();
                    handle_call(
                        program,
                        function,
                        Some((&result, data_type)),
                        location,
                        path,
                        arguments,
                    );
                    function.operations.store_from_pointer(
                        &data_type.convert(),
                        &result,
                        destination,
                    )
                } else {
                    handle_call(
                        program,
                        function,
                        Some((destination, data_type)),
                        location,
                        path,
                        arguments,
                    );
                }
            } else {
                let result = function.variables.increment();
                handle_call(
                    program,
                    function,
                    Some((&result, &data_type)),
                    location,
                    path,
                    arguments,
                );
                return IRValue::Variable(result);
            }
            return IRValue::Null;
        }
        _ => todo!("{info:#?}"),
    };

    if let Some(destination) = destination {
        if should_allocate {
            function
                .operations
                .allocate(&destination, &data_type.convert())
        }

        function
            .operations
            .store(&data_type.convert(), &value, &destination);
    };

    return value;
}
