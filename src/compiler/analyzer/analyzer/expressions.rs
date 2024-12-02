use crate::compiler::{
    analyzer::{IRValue, Operation},
    parser::{Expression, ExpressionInfo, Value},
    types::{BaseType, Type},
};

use super::{FunctionCtx, ProgramCtx};

fn void() -> (IRValue, Type) {
    return (IRValue::Null, Type::default());
}

impl BaseType {
    fn is_integer(&self) -> bool {
        matches!(
            &self,
            Self::Int8
                | Self::Int16
                | Self::Int32
                | Self::Int64
                | Self::UInt8
                | Self::UInt16
                | Self::UInt32
                | Self::UInt64
        )
    }
    fn is_bool(&self) -> bool {
        matches!(&self, Self::Boolean)
    }
}

fn what_type(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    expected_type: Option<&Type>,
    expression: &ExpressionInfo,
) -> Type {
    let mut data_type: Type = match &expression.expression {
        Expression::Value(value) => match expected_type {
            Some(expected) => {
                if match value {
                    Value::Integer(_) => expected.base.is_integer(),
                    Value::Boolean(_) => expected.base.is_bool(),
                    _ => todo!(),
                } {
                    expected.clone()
                } else {
                    value.default_type()
                }
            }

            None => value.default_type(),
        },
        Expression::GetVariable(path) => {
            let name = path.first().unwrap();
            let variable = match function.variables.read(name) {
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
        },
        Expression::BinaryOperation(a, _, b) => {
            let first = a.as_ref();
            let second = b.as_ref();

            let data_type = what_type(program, function, expected_type, first);
            let data_type = what_type(program, function, Some(&data_type), second);

            data_type
        }
        _ => todo!("{:#?}", expression),
    };
    data_type.ref_state = expression.ref_state.clone();
    return data_type;
}

pub fn handle_expression(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    expected_type: &Option<Type>,
    expression: Option<ExpressionInfo>,
) -> (IRValue, Type) {
    let expression = match expression {
        Some(expression) => expression,
        None => return void(),
    };

    let expected_type: Type = match expected_type {
        Some(t) => {
            let expected = t.clone();
            let infered = what_type(program, function, Some(&expected), &expression);

            if infered != expected {
                program.debug.error(
                    expression.location.clone(),
                    format!("Wrong types, expected: '{expected}' but got: '{infered}'"),
                );
            }

            expected
        }
        None => what_type(program, function, None, &expression),
    };

    let value = match expression.expression {
        Expression::Value(value) => match value {
            Value::Integer(int) => IRValue::IntLiteral(int),
            Value::Boolean(bool) => IRValue::BoolLiteral(bool),
            _ => todo!(),
        },
        Expression::BinaryOperation(a, operator, b) => {
            let result = function.variables.increment();

            let first = *a;
            let second = *b;

            let ir = expected_type.convert();
            let et = Some(expected_type.clone());

            let (first_value, _) = handle_expression(program, function, &et, Some(first));
            let (second_value, _) = handle_expression(program, function, &et, Some(second));

            function.operations.push(Operation::BinaryOperation { float: false, destination: result.clone(), operator, data_type: ir, first: first_value, second: second_value });
            IRValue::Variable(result)
        },
        Expression::GetVariable(path) => {
            let name = path.first().unwrap();
            let result_key = function.variables.increment();

            let variable = match function.variables.read(name) {
                Some(var) => var,
                None => {
                    program.debug.error(
                        expression.location,
                        format!("Could not find variable named: '{name}'"),
                    );
                    return void();
                }
            };

            if variable.is_parameter {
                IRValue::Variable(variable.key.clone())
            } else {
                function.operations.push(Operation::Load {
                    destination: result_key.clone(),
                    destination_type: expected_type.convert(),
                    value: IRValue::Variable(variable.key.clone()),
                });
                IRValue::Variable(result_key)
            }
        }
        Expression::Call(path, mut arguments) => {
            let result_key = function.variables.increment();
            let found = match program.types.get_function(function.relative_path, &path) {
                Some(f) => f,
                None => {
                    program.debug.error(
                        expression.location,
                        format!("Could not find function: '{path}'"),
                    );
                    return void();
                }
            };

            if arguments.len() != found.parameters.len() {
                program.debug.error(
                    expression.location,
                    format!(
                        "Expected {} arguments, but got {}",
                        found.parameters.len(),
                        arguments.len()
                    ),
                );
                return void();
            }

            arguments.reverse();

            let mut ir_arguments = Vec::new();
            for param_type in &found.parameters {
                let expression = arguments.pop();
                let (value, data_type) =
                    handle_expression(program, function, param_type, expression);

                ir_arguments.push((data_type.convert(), value));
            }

            function.operations.push(Operation::StoreCall {
                destination: result_key.clone(),
                function: found.key.clone(),
                return_type: found.return_type.convert(),
                arguments: IRValue::Arguments(ir_arguments),
            });
            
            IRValue::Variable(result_key)
        }
        _ => {
            program
                .debug
                .result_print(format!("{:#?}", expression.expression));
            return void();
        }
    };

    return (value, expected_type);
}
