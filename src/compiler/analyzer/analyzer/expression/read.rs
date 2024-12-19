use crate::compiler::{
    analyzer::{FunctionCtx, IRValue, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
    types::Type,
};

use super::handle_expression;

pub fn handle_read(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    data_type: &Type,
    info: ExpressionInfo,
) -> IRValue {
    let destination = function.variables.increment();
    handle_expression(program, function, location, &destination, false, data_type, info)
}
