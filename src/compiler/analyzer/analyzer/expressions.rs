use crate::compiler::{
    analyzer::IRValue,
    parser::{Expression, ExpressionInfo, Value},
    types::Type,
};

use super::{FunctionCtx, ProgramCtx};

fn void() -> (IRValue, Type) {
    return (IRValue::Null, Type::void());
}

fn parse_type(
    _program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    expression: &ExpressionInfo,
) -> Type {
    return match &expression.expression {
        Expression::Value(value) => value.default_type(),
        Expression::GetVariable(path) => {
            let name = path.first().unwrap();
            let variable = match function.variables.read(&name) {
                Some(var) => var,
                None => todo!(),
            };

            variable.data_type.clone().unwrap()
        }
        _ => todo!(),
    };
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

    let infered_type = match expected_type {
        Some(t) => t.clone(),
        None => parse_type(program, function, &expression),
    };

    match expression.expression {
        Expression::GetVariable(path) => {
            let name = path.first().unwrap();

            match function.variables.is_borrowed(name) {
                Some(bool) => if bool {
                    program.debug.error(
                        expression.location,
                        format!("Cannot use after borrow: '{name}'"),
                    );
                    return void();
                },
                None => {
                    program.debug.error(
                        expression.location,
                        format!("Could not find variable named: '{name}'"),
                    );
                    return void();
                }
            }

            let variable = function.variables.borrow(&name).unwrap();


            (
                IRValue::Variable(variable.key.clone()),
                variable.data_type.clone().unwrap(),
            )
        }
        Expression::Value(value) => (
            match value {
                Value::Integer(int) => IRValue::IntLiteral(int),
                _ => todo!(),
            },
            infered_type,
        ),
        _ => todo!("{:#?}", expression.expression),
    }
}
