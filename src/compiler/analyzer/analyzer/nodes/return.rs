use crate::compiler::{
    analyzer::{analyzer::handle_expression, FunctionCtx, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
};

pub fn handle_return(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    _location: Location,
    expression: Option<ExpressionInfo>,
) {
    let (value, data_type) = handle_expression(program, function, function.return_type, expression);

    function.operations.r#return(&data_type.convert(), &value);
}
