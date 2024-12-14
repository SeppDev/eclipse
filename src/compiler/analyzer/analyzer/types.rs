use crate::compiler::{
    errors::Location,
    parser::{Expression, ExpressionInfo, Value},
    types::{BaseType, ReferenceState, Type},
};

use super::{FunctionCtx, ProgramCtx};

pub fn what_type(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    expected_type: Option<&Type>,
    expression: &ExpressionInfo,
) -> Type {
    let mut infered_type: Type = match &expression.expression {
        Expression::Index(path, _) => {
            let name = path;
            let array = match function.variables.read(name, &ReferenceState::Mutable) {
                Some(var) => var,
                None => return Type::void(),
            };
            if !array.data_type.base.is_array() {
                program.debug.error(
                    expression.location.clone(),
                    "Cannot index into a value type",
                );
            }

            let (data_type, _) = array.data_type.array_info();
            return data_type.clone();
        }
        Expression::Array(array) => {
            let size = array.len();
            let first = array.first().unwrap();

            let inner_type = if let Some(expected) = expected_type {
                if !expected.base.is_array() {
                    program.debug.error(
                        location.clone(),
                        format!("Expected array but got {expected}"),
                    );
                    return expected.clone();
                }
                let (value_type, _) = expected.array_info();
                what_type(program, function, location, Some(value_type), first)
            } else {
                what_type(program, function, location, None, first)
            };

            let infered = Type::new(BaseType::Array(size, Box::new(inner_type)));

            infered
        }
        Expression::Value(value) => match expected_type {
            Some(expected) => {
                if match value {
                    Value::Integer(_) => expected.base.is_integer(),
                    Value::Boolean(_) => expected.base.is_bool(),
                    Value::Float(_) => expected.base.is_float(),
                    _ => todo!(),
                } {
                    expected.clone()
                } else {
                    value.default_type()
                }
            }

            None => value.default_type(),
        },
        Expression::GetVariable(name) => {
            let variable = match function.variables.read(name, &ReferenceState::None) {
                Some(var) => var,
                None => {
                    program.debug.error(
                        expression.location.clone(),
                        format!("Could not find variable named: '{name}'"),
                    );
                    return Type::void();
                }
            };
            variable.data_type.clone()
        }
        Expression::Call(path, _) => {
            let found = match program.types.get_function(function.relative_path, path) {
                Some(f) => f,
                None => return Type::void(),
            };
            found.return_type.clone()
        }
        Expression::BinaryOperation(a, _, b) => {
            let first = a.as_ref();
            let second = b.as_ref();

            let data_type = what_type(program, function, location, expected_type, first);
            if !data_type.base.is_number() {
                program.debug.error(
                    first.location.clone(),
                    "Number is required for this operator",
                );
            }

            let data_type_second = what_type(program, function, location, Some(&data_type), second);
            if !data_type_second.base.is_number() {
                program.debug.error(
                    second.location.clone(),
                    "Number is required for this operator",
                );
            }

            if data_type != data_type_second {
                program.debug.error(
                    expression.location.clone(),
                    format!(
                        "Operation types haves to be the same: {data_type} != {data_type_second}"
                    ),
                );
            }

            data_type
        }
        Expression::CompareOperation(a, _, b) => {
            let first = a.as_ref();
            let second = b.as_ref();

            let data_type = what_type(program, function, location, expected_type, first);
            let data_type_second = what_type(program, function, location, Some(&data_type), second);
            if data_type != data_type_second {
                program.debug.error(
                    expression.location.clone(),
                    format!(
                        "Operation types haves to be the same: {data_type} != {data_type_second}"
                    ),
                );
            }

            Type::new(BaseType::Boolean)
        }
        Expression::Minus(info) => {
            let data_type = what_type(program, function, location, expected_type, info);
            if !data_type.base.is_number() {
                program.debug.error(
                    info.location.clone(),
                    "Number is required for this operation",
                );
            }
            data_type
        }
        Expression::Not(info) => {
            let data_type = what_type(
                program,
                function,
                location,
                Some(&Type::new(BaseType::Boolean)),
                info,
            );
            if !data_type.base.is_bool() {
                program.debug.error(
                    info.location.clone(),
                    "Boolean is required for this operation",
                );
            }
            data_type
        }
        _ => todo!("{:#?}", expression),
    };
    infered_type.ref_state = expression.ref_state.clone();

    if let Some(expected) = expected_type {
        if expected != &infered_type {
            program.debug.error(
                location.clone(),
                format!("Expected {expected} but got {infered_type}"),
            );
        }
    }

    return infered_type;
}
