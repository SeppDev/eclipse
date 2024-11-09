use crate::compiler::{
    parser::{ExpressionInfo, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::{BaseType, Type},
};

pub fn analyze(program: ParsedProgram) {
    let std = &program.standard;
    analyze_file(&program, std, &Path::from("std"));

    let main = &program.main;
    analyze_file(&program, main, &Path::from("src").join("main"));
}

fn analyze_file(program: &ParsedProgram, file: &ParsedFile, path: &Path) {    
    for (name, function) in &file.functions {
        analyze_body(program, file, path, &function.return_type, &function.body);
    }

    for (name, imported) in &file.imported {
        analyze_file(program, imported, &path.join(name));
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
        let _ = match &info.node {
            Node::Return(expression) => {
                analyze_expression(program, file, namespace, Some(return_type.clone()), expression)
                    .or_else(|| {
                        file.throw_error(
                            format!(
                                "Expected '{}', but returned '{}'",
                                return_type,
                                BaseType::Void
                            ),
                            &info.location,
                        )
                    });
            }
            Node::Variable { name, mutable, data_type, expression } => {
                analyze_expression(program, file, namespace, data_type, &Some(expression.clone()));
            }
            _ => file.throw_error("Unhandled node", &info.location),
        };
    }
}


fn analyze_expression(
    program: &ParsedProgram,
    file: &ParsedFile,
    namespace: &Path,
    return_type: &Option<Type>,
    expression: &Option<ExpressionInfo>,
) -> Option<()> {
    let expression = match expression {
        Some(expr) => expr,
        None => return None,
    };

    
}
