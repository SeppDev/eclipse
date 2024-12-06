use crate::compiler::{
    analyzer::{FunctionCtx, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo, types::Type,
};

use super::handle_read;

pub fn handle_return(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    return_type: &Option<Type>,
    expression: Option<ExpressionInfo>,
) {
    let expression = match expression {
        Some(e) => e,
        None => {
            function.operations.void_return();
            return;
        }
    };
    let data_type = match return_type {
        Some(d) => d,
        None => &Type::void()
    };

    let value = handle_read(program, function, &location, data_type, expression);
    function.operations.r#return(&data_type.convert(), &value);
}
