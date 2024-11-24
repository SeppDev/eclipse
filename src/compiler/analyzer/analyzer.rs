use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult, Location, MessageKind},
    parser::{Expression, ExpressionInfo, Node, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::{BaseType, Type},
};

use super::{
    parse_types,
    variables::Variables,
    FileTypes, IRProgram, IRType,
};


pub fn analyze(
    debug: &mut CompileCtx, 
    count: &mut NameCounter,
    types: FileTypes,
    program: ParsedProgram,
) -> CompileResult<IRProgram> {
    let functions = Vec::new();
    // let std_path = Path::from("std"); 
    // analyze_file(parsed, &mut functions, errors, &parsed.standard, &std_path);

    // handle_file(debug, &mut functions, &types, program.main)?;

    return Ok(IRProgram { functions });
}

fn handle_file(debug: &mut CompileCtx, ) {

}
