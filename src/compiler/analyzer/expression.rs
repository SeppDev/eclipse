use crate::compiler::{errors::CompileResult, parser::{Expression, ExpressionInfo, Value}, types::Type};

use super::{variables::{Variables}, IRValue, Operation, ProgramCtx};

pub fn handle_expression(
    program: &mut ProgramCtx,
    operations: &mut Vec<Operation>,
    variables: &mut Variables,
    return_type: &Option<Type>,
    expression: Option<ExpressionInfo>,
) -> CompileResult<(IRValue, Type)> {
    let info = match expression {
        Some(info) => info,
        None => return Ok((IRValue::Null, Type::void())),
    };

    let expected_type = match return_type {
        Some(t) => t,
        None => {
            todo!()
        },
    };

    return Ok(match info.expression {
        Expression::Value(value) => match value {
            Value::Integer(int) => {
                if expected_type.is_integer() {
                    program.debug.error(, location, message)
                    return Ok((IRValue::Null, Type::void()))
                };
                (IRValue::IntLiteral(int), expected_type.clone())
            },
            _ => todo!("{:?}", value),
        },
        Expression::GetVariable(path) => {
            let key = path.first().unwrap();
            let location = variables.increment();
            let variable = match variables.get(key, false) {
                Some(var) => var,
                None => todo!()
            };

            operations.push(Operation::Load(location.clone(), variable.data_type.convert(), variable.name.clone()));

            (IRValue::Variable(location), expected_type.clone())
        }
        _ => todo!("{:#?}", info),
    });
}