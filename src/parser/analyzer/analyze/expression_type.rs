use std::collections::HashMap;

use eclipse::BuildError;

use crate::{analyzer::analyzer::call::call, parser::{Expression, Type, Value}};

use super::{Function, Scope};

pub fn expression_type(
    expression: Expression,
    scope: &Scope,
    function: &Function,
    functions: &HashMap<String, Function>,
) -> Result<Type, BuildError> {
    use crate::parser::Integer;
    use crate::parser::Type;

    let t = match expression {
        Expression::Value(value, _) => match value {
            Value::Boolean(_) => Type::Boolean,
            Value::String(_) => Type::String,
            Value::Integer(_) => Type::Integer(Integer::i64),
        },
        Expression::GetVariable(name) => {
            let variable = match scope.variables.get(&name) {
                Some(a) => a,
                None => return Err(BuildError::NotDefined(name)),
            };
            variable.var_type.clone()
        }
        Expression::Call(name, arguments) => match functions.get(&name) {
            Some(function) => {
                match call(function, arguments, scope, functions) {
                    Some(error) => return Err(error),
                    None => {}
                };
                match function.return_type.clone() {
                    Some(t) => t,
                    None => return Err(BuildError::WrongReturnType),
                }
            }
            None => return Err(BuildError::NotDefined(name)),
        },
        Expression::BinaryOperation(a, _, b) => {
            let a = match expression_type(*a, scope, function, functions) {
                Ok(a) => a,
                Err(error) => return Err(error),
            };
            let b = match expression_type(*b, scope, function, functions) {
                Ok(a) => a,
                Err(error) => return Err(error),
            };

            if a != b {
                return Err(BuildError::WrongReturnType);
            }

            return Ok(a);
        }
    };

    return Ok(t);
}