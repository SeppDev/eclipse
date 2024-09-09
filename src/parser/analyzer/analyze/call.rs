use std::collections::HashMap;

use eclipse::BuildError;

use crate::parser::Expression;

use super::{expression_type::expression_type, Function, Scope};

pub fn call(
    function: &Function,
    arguments: Vec<Expression>,
    scope: &Scope,
    functions: &HashMap<String, Function>,
) -> Option<BuildError> {
    if arguments.len() != function.parameters.len() {
        return Some(BuildError::TooFewOrManyArguments);
    }

    for (index, argument) in arguments.into_iter().enumerate() {
        let (_, t) = function.parameters.get(index).unwrap();
        let a = match expression_type(argument, scope, function, functions) {
            Ok(a) => a,
            Err(error) => return Some(error),
        };
        if a != t.to_owned() {
            return Some(BuildError::WrongType);
        }
    }

    None
}