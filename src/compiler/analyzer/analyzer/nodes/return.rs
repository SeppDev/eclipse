use crate::compiler::{
    analyzer::{analyzer::handle_expression, FunctionCtx, Operation, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
};

pub fn handle_return(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    _location: Location,
    expression: Option<ExpressionInfo>,
) {
    let (value, data_type) = handle_expression(program, function, function.return_type, false, expression);

    function.operations.push(Operation::Return {
        data_type: data_type.convert(),
        value,
    });
}
