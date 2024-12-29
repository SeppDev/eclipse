use crate::compiler::{
    analyzer::{
        analyzer::{handle_body, LoopInfo},
        handle_expression, FunctionCtx, ProgramCtx,
    },
    errors::Location,
    parser::{ExpressionInfo, NodeInfo},
    types::Type,
};

pub fn handle_loop(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    return_type: &Option<Type>,
    location: Location,
    condition: Option<ExpressionInfo>,
    body: Vec<NodeInfo>,
) {
    let begin = function.increment_key();
    let end = function.increment_key();
    function
        .loop_info
        .push(LoopInfo::new(begin.clone(), end.clone()));

    function.operations.goto(&begin);
    function.operations.label(&begin);

    match condition {
        Some(expression) => {
            let result = handle_expression(
                program,
                function,
                &location,
                None,
                &Type::boolean(),
                expression,
            );

            let after = function.increment_key();
            function.operations.branch(&result, &after, &end);
            function.operations.label(&after);
        }
        None => {}
    }

    handle_body(program, function, return_type, body);

    function.operations.goto(&begin);
    function.operations.label(&end);

    let _ = function.loop_info.pop();
}

pub fn handle_break(program: &mut ProgramCtx, function: &mut FunctionCtx, location: Location) {
    let last = match function.loop_info.last() {
        Some(li) => li,
        None => {
            program
                .debug
                .error(location, "Break can only be used inside of loops");
            return;
        }
    };
    function.operations.goto(&last.end);
}

pub fn handle_continue(program: &mut ProgramCtx, function: &mut FunctionCtx, location: Location) {
    let last = match function.loop_info.last() {
        Some(li) => li,
        None => {
            program
                .debug
                .error(location, "Continue can only be used inside of loops");
            return;
        }
    };
    function.operations.goto(&last.begin);
}
