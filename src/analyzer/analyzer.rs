use std::collections::HashMap;

use crate::{
    analyzer::nodes::IRFunction, ASTModule, ASTNode, AnalyzeResult, Expression, Node, Path, Type,
    Value,
};

use super::{
    get_function_types, nodes::{IRExpression, IRNode}, variables::Variables, ModuleTypes, IRModule, IRProgram
};

pub fn analyze(module: ASTModule) -> AnalyzeResult<IRProgram> {
    let types = get_function_types(&module)?;

    let main_path = Path::from(String::from("main"));
    let mut modules = HashMap::new();
    handle_module(&mut modules, module, &types, main_path)?;

    return Ok(IRProgram { modules, types });
}

fn handle_module(
    modules: &mut HashMap<Path, IRModule>,
    module: ASTModule,
    types: &ModuleTypes,
    current_path: Path,
) -> AnalyzeResult<()> {
    let mut functions = HashMap::new();

    for node in module.body {
        match node.node {
            Node::Function {
                export,
                is_unsafe,
                name,
                generics,
                parameters,
                return_type,
                body,
            } => {
                let mut variables = Variables::new(parameters.clone()); 
                hanlde_scope(types, &current_path, nodes);
       
                // let function = IRFunction {
                //     parameters,
                //     return_type,
                //     nodes,
                // };
                // functions.insert(name, function);
            }
            _ => continue,
        }
    }

    for (name, (_, ast_module)) in module.submodules {
        let mut sub_path = current_path.clone();
        sub_path.add(name.clone());

        handle_module(modules, ast_module, types, sub_path)?;
    }

    modules.insert(current_path, IRModule { functions });
    return Ok(());
}

fn hanlde_scope(types: &ModuleTypes, current_path: &Path, nodes: Vec<ASTNode>) {

}