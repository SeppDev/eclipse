use crate::compiler::{
    analyzer::{analyzer::{handle_body, LoopInfo}, handle_read, FunctionCtx, ProgramCtx},
    errors::Location,
    parser::{ExpressionInfo, NodeInfo}, types::{BaseType, Type},
};


pub fn handle_loop(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    condition: Option<ExpressionInfo>,
    body: Vec<NodeInfo>,
) {
    let begin = function.variables.increment();
    let end = function.variables.increment();
    function.loop_info.push(LoopInfo::new(begin.clone(), end.clone()));

    function.operations.goto(&begin);
    function.operations.label(&begin);


    match condition {
        Some(expression) => {
            let result = handle_read(
                program,
                function,
                &location,
                &Type::new(BaseType::Boolean),
                expression,
            );
            let after = function.variables.increment();
            function.operations.branch(&result, &after, &end);
            function.operations.label(&after);
        },
        None => {}
    }

    handle_body(program, function, body);

    function.operations.goto(&begin);
    function.operations.label(&end);

    let _ = function.loop_info.pop();
}

pub fn handle_break(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location
) {
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

pub fn handle_continue(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location
) {
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
