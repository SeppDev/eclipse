use crate::{
    analyzer::{FunctionTypes, IRModule, IRNode},
    codegen::builder::Builder,
    Path,
};

use super::builder::ScopeBuilder;

pub fn generate(main: IRModule, types: FunctionTypes) {
    println!("{:#?}", types);
    println!("{:#?}", main);

    let mut builder = Builder::new();

    codegen(&mut builder, main, Path::from(String::from("main")));

    println!("{:#?}", builder);
}

fn codegen(builder: &mut Builder, module: IRModule, path: Path) {
    for (name, function) in module.body {
        let mut function_path = path.clone();
        function_path.add(name);

        let scope = builder.function(function_path);
        handle_scope(scope, function.nodes);
    }

    for (name, (_, module)) in module.submodules {
        let mut new_path = path.clone();
        new_path.add(name);
        codegen(builder, module, new_path);
    }
}

fn handle_scope(scope: &mut ScopeBuilder, nodes: Vec<IRNode>) {
    for node in nodes {
        match node {
            IRNode::DefineVariable { name, expression } => {
                match expression {
                    
                }
            },
            IRNode::Return(expression) => {}
            IRNode::Scope { is_unsafe, body } => handle_scope(scope.create(), body),
            _ => todo!(),
        }
    }
}
