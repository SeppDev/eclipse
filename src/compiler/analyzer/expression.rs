use crate::compiler::{
    errors::{CompileResult, Location},
    parser::{Expression, ExpressionInfo, Value},
    path::Path,
    types::Type,
};

use super::{variables::Variables, IRValue, Operation, ProgramCtx};

pub fn handle_expression(
    program: &mut ProgramCtx,
    operations: &mut Vec<Operation>,
    variables: &mut Variables,
    relative_path: &Path,
    return_type: &Option<Type>,
    location: &Location,
    expression: Option<ExpressionInfo>,
) -> CompileResult<(IRValue, Type)> {
    let info = match expression {
        Some(info) => info,
        None => match return_type {
            Some(rt) => {
                if rt.is_void() {
                    return Ok((IRValue::Null, Type::void()));
                } else {
                    program.debug.error(
                        location.clone(),
                        format!("Expected type '{rt}', got 'null'"),
                    );
                    return Ok((IRValue::Null, Type::void()));
                }
            }
            None => return Ok((IRValue::Null, Type::void())),
        },
    };

    let expected_type = match return_type {
        Some(t) => t,
        None => &what_type(&info, variables, &program, relative_path)?,
    };

    return Ok(match info.expression {
        Expression::Value(value) => match value {
            Value::Integer(int) => {
                if !expected_type.is_integer() {
                    program.debug.error(
                        info.location.clone(),
                        format!("Expected type {expected_type}, got 'integer'"),
                    );
                    return Ok((IRValue::Null, Type::void()));
                };
                (IRValue::IntLiteral(int), expected_type.clone())
            }
            Value::Boolean(bool) => {
                if !expected_type.is_bool() {
                    program.debug.error(
                        info.location.clone(),
                        format!("Expected type '{expected_type}', got 'boolean'"),
                    );
                    return Ok((IRValue::Null, Type::void()));
                };
                (IRValue::BoolLiteral(bool), expected_type.clone())
            }
            Value::Float(float) => {
                if !expected_type.is_float() {
                    program.debug.error(
                        info.location.clone(),
                        format!("Expected type '{expected_type}', got 'float'"),
                    );
                    return Ok((IRValue::Null, Type::void()));
                };
                (IRValue::FloatLiteral(float), expected_type.clone())
            }
            _ => todo!("{:?}", value),
        },
        Expression::GetVariable(path) => {
            let key = path.first().unwrap();
            let location = variables.increment();
            let variable = match variables.get(key) {
                Some(var) => var,
                None => todo!(),
            };
            let data_type = variable.data_type.as_ref().unwrap();

            if data_type != expected_type {
                program.debug.error(
                    info.location.clone(),
                    format!("Expected type '{expected_type}', got '{data_type}'"),
                );
            }

            operations.push(Operation::Load(
                location.clone(),
                data_type.convert(),
                data_type.convert().pointer(),
                variable.key.clone()
            ));

            (IRValue::Variable(location), expected_type.clone())
        }
        Expression::Call(path, mut arguments) => {
            let found = match program.types.get_function(relative_path, &path)? {
                Some(f) => f,
                None => todo!(),
            };
            let to = variables.increment();

            if arguments.len() != found.parameters.len() {
                program.debug.error(
                    info.location.clone(),
                    format!(
                        "Expected {} arguments, but got {}",
                        found.parameters.len(),
                        arguments.len()
                    ),
                );
                return Ok((IRValue::Null, expected_type.clone()));
            }
            arguments.reverse();

            let mut ir_arguments = Vec::new();
            
            // operations.push(Operation::Allocate(to.clone(), expected_type.convert()));

            for param_type in &found.parameters {
                let expression = arguments.pop();
                let (value, data_type) = handle_expression(
                    program,
                    operations,
                    variables,
                    relative_path,
                    param_type,
                    location,
                    expression,
                )?;

                ir_arguments.push((data_type.convert(), value));
            }

            operations.push(Operation::StoreCall(
                to.clone(),
                found.key.clone(),
                found.return_type.convert(),
                IRValue::Arguments(ir_arguments),
            ));
            (IRValue::Variable(to), found.return_type.clone())
        }
        _ => todo!("{:#?}", info),
    });
}

fn what_type(
    info: &ExpressionInfo,
    variables: &Variables,
    program: &ProgramCtx,
    relative_path: &Path,
) -> CompileResult<Type> {
    return Ok(match &info.expression {
        Expression::Value(value) => value.default_type(),
        Expression::GetVariable(path) => {
            let key = path.first().unwrap();
            let variable = variables.get(key).unwrap();
            variable.data_type.clone().unwrap()
        }
        Expression::Call(path, _) => {
            let found = match program.types.get_function(relative_path, &path)? {
                Some(f) => f,
                None => todo!(),
            };
            found.return_type.clone()
        }
        _ => todo!(),
    });
}
