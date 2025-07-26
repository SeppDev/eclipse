use common::string::Appendable;
use context::CompilerCtx;
use syntax::mir::{Expression, Module, Node, Parameter, Type};

use std::fmt::Write;

mod llvm;

pub fn generate(compiler: &CompilerCtx, module: Module) -> String {
    let mut source = String::new();

    source.pushln("target triple = \"x86_64-pc-unix-unkown\"");

    for node in module.nodes {
        node_to_string(node, &mut source);
    }

    // source.pushln("define i32 @main() {");
    // source.pushln("start:");
    // source.pushln("ret i32 2");
    // source.pushln("}");

    source
}

fn function(
    source: &mut String,
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Vec<Node>,
) {
    let _ = write!(source, "define void @{name}() {{\n");

    for node in body {
        node_to_string(node, source);
    }

    source.pushln("}");
}

fn node_to_string(node: Node, source: &mut String) {
    use Node::*;

    match node {
        Return(expr) => match expr {
            Some(expr) => todo!(),
            None => source.push_str("ret void"),
        },
        Allocate(data_type) => todo!(),
        Block(nodes) => todo!(),
        DeclareVariable {
            name,
            data_type,
            value,
        } => todo!(),
        Function {
            name,
            parameters,
            return_type,
            body,
        } => function(source, name, parameters, return_type, body),
    };

    source.line();
}

fn expression_to_string(expression: Expression, source: &mut String) -> String {
    use Expression::*;

    match expression {
        Integer(n) => todo!(),
    }
}
