use crate::{AnalyzeResult, CompileError, Expression, Type, Value};

use super::{types::functions::ModuleTypes, variables::Variables};

pub fn define_variable(
    types: &ModuleTypes,
    variables: &Variables,
    expression: Expression,
) -> AnalyzeResult<Type> {
    use crate::BaseType::*;

    match expression {
        Expression::GetVariable(name) => {
            let variable = variables.get(&name)?;
            match &variable.data_type {
                Some(t) => return Ok(t.clone()),
                None => todo!("Missing type to inherit from: {}", name),
            }
        }
        Expression::Value(value) => match value {
            Value::Integer(signed, int) => return Ok(Type::Base(Int32)),
            Value::Boolean(_) => return Ok(Type::Base(Boolean)),
            v => todo!("{:#?}", v),
        },
        e => todo!("{:?}", e),
    }
}

// Some(t) => {
//     if t.is_integer() {
//         t.clone()
//     } else {
//         return Err(CompileError::new(format!("Wrong return type"), 0));
//     }
// }