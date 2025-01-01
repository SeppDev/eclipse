// pub use array::handle_store;
// pub use read::handle_read;

use crate::compiler::{
    analyzer::{FunctionCtx, IRType, IRValue},
    errors::Location,
    parser::{Expression, ExpressionInfo, Value},
    types::{BaseType, Type},
    POINTER_WITH,
};

mod array;
pub use array::handle_array_store;

use super::{program::ProgramCtx, what_type};

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
    data_type: &Type,
    info: ExpressionInfo,
) -> IRValue {
    let value = match info.expression {
        Expression::Value(_) if data_type.pointers() > 0 => {
            let result = function.increment_key();
            let deref = &data_type.clone().dereference().unwrap();

            function.operations.allocate(&result, &deref.convert());
            handle_expression(
                program,
                function,
                location,
                Some(&result),
                &data_type.clone().dereference().unwrap(),
                info,
            );

            IRValue::Variable(result)
        }
        Expression::Value(value) => match value {
            Value::Integer(int) => IRValue::IntLiteral(int),
            Value::Boolean(bool) => IRValue::BoolLiteral(bool),
            Value::Float(float) => IRValue::FloatLiteral(float),
            _ => todo!(),
        },
        Expression::Reference(expression) => {
            return handle_expression(
                program,
                function,
                location,
                destination,
                data_type,
                *expression,
            )
        }
        Expression::DeReference(expression) => {
            let pointer = function.increment_key();
            let variable = handle_expression(
                program,
                function,
                location,
                None,
                &Type::pointer(data_type.base.clone()),
                *expression,
            );
            function
                .operations
                .load(&pointer, &IRType::Pointer, &variable);

            let result = function.increment_key();
            function
                .operations
                .load_from_pointer(&result, &data_type.convert(), &pointer);
            IRValue::Variable(result)
        }
        Expression::GetVariable(name) if data_type.pointers() > 0 => {
            let variable = function.read_variable(&name).unwrap();
            IRValue::Variable(variable.key.clone())
        }
        Expression::GetVariable(name) => {
            let result = function.increment_key();
            let variable = function.read_variable(&name).unwrap().clone();

            function
                .operations
                .load_from_pointer(&result, &data_type.convert(), &variable.key);
            IRValue::Variable(result)
        }
        Expression::Field(value, field) => {
            let pointer = function.increment_key();
            let object = handle_expression(program, function, location, Some(&pointer), data_type, *value);
            return IRValue::Null;
        }
        Expression::Struct(_, fields) => {
            let structure = if let BaseType::Struct(structure) = &data_type.base {
                structure
            } else {
                todo!()
            };
            let destination = if let Some(destination) = destination {
                destination
            } else {
                todo!()
            };

            for (key, value) in fields {
                let (field_type, offset) = structure.get_info(&key).unwrap();
                let pointer = function.increment_key();
                function.operations.getelementptr_inbounds(
                    &pointer,
                    &data_type.convert(),
                    destination,
                    &IRType::Integer(POINTER_WITH),
                    &IRValue::IntLiteral(offset.to_string()),
                );

                let field_type = what_type(program, function, location, Some(field_type), &value);
                handle_expression(
                    program,
                    function,
                    location,
                    Some(&pointer),
                    &field_type,
                    value,
                );
            }

            return IRValue::Null;
        }
        expression => {
            program
                .debug
                .error(location.clone(), format!("TODO: {expression:#?}"));
            return IRValue::Null;
        }
    };

    if let Some(destination) = destination {
        function
            .operations
            .store(&data_type.convert(), &value, &destination);
    };

    return value;
}
