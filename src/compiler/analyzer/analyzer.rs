use crate::compiler::{
    parser::{ExpressionInfo, Function, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

pub fn analyze(program: ParsedProgram) {
    let std = &program.standard;
    analyze_file(&program, std, &Path::from("std"));

    let main = &program.main;
    analyze_file(&program, main, &Path::from("src").join("main"));
}

fn analyze_file(program: &ParsedProgram, file: &ParsedFile, path: &Path) {
    for (name, imported) in &file.imported {
        analyze_file(program, imported, &path.join(name));
    }

    for (name, function) in &file.functions {
        analyze_body(program, file, path, &function.return_type, &function.body);
    }
}

fn analyze_body(
    program: &ParsedProgram,
    file: &ParsedFile,
    namespace: &Path,
    return_type: &Type,
    nodes: &Vec<NodeInfo>,
) {
    use super::super::parser::Node;

    for info in nodes {
        match &info.node {
            Node::Return(expression) => {
                analyze_expression(program, file, namespace, Some(return_type), expression)
                    .or_else(|| {
                        file.throw_error(
                            format!("Expected '{}', but returned void", return_type),
                            &info.location,
                        )
                    });
            }
            _ => file.throw_error("Unhandled node", &info.location),
        }
    }
}

fn analyze_expression(
    program: &ParsedProgram,
    file: &ParsedFile,
    namespace: &Path,
    return_type: Option<&Type>,
    expression: &Option<ExpressionInfo>,
) -> Option<()> {
    let expression = match expression {
        Some(expr) => expr,
        None => return None,
    };

    None
}
