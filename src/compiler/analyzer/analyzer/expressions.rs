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
        _ => todo!(),
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

    let infered_type: Type = match expected_type {
        Some(t) => {
            let specified = t.clone();
            let expected = what_type(program, function, Some(&specified), &expression);

            if specified != expected {
                program.debug.error(
                    expression.location.clone(),
                    format!("Wrong types, expected: '{specified}' but got: '{expected}'"),
                );
                return void();
            }

            specified
        }
        None => what_type(program, function, None, &expression),
    };

    let value = match expression.expression {
        Expression::Value(value) => match value {
            Value::Integer(int) => IRValue::IntLiteral(int),
            _ => todo!(),
        },
        Expression::GetVariable(path) => {
            let name = path.first().unwrap();
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

            
            IRValue::Variable(variable.key.clone())
        }
        _ => {
            program
                .debug
                .result_print(format!("{:#?}", expression.expression));
            return void();
        }
    };

    return (value, infered_type);
}
