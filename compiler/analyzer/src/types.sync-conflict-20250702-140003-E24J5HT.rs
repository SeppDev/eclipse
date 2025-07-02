use std::collections::HashMap;

use diagnostics::DiagnosticResult;
use shared::path::Path;
use syntax::ast;

use super::Analyzer;

#[derive(Debug)]
pub struct FunctionType {
    // generics: Vec<String>,
    parameters: Vec<ast::Parameter>,
    return_type: ast::Type,
}

#[derive(Debug, Default)]
pub struct ModuleTypes {
    // pub types: HashMap<String, Type>,
    pub functions: HashMap<String, FunctionType>,
}

#[derive(Debug, Default)]
pub struct Types {
    pub modules: HashMap<Path, ModuleTypes>,
}
impl Types {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Analyzer {
    fn get_module_types(&self, module: &ast::Module) -> DiagnosticResult<ModuleTypes> {
        let mut module_types = ModuleTypes::default();
        let functions = &mut module_types.functions;
        // let types = &mut module_types.types;

        for node in &module.body {
            match &node.raw {
                ast::RawNode::Function {
                    name,
                    parameters,
                    return_type,
                    ..
                } => functions.insert(
                    name.raw.clone(),
                    FunctionType {
                        parameters: parameters.clone(),
                        return_type: return_type.clone(),
                    },
                ),
                raw => todo!("{raw:#?}"),
            };
        }

        Ok(module_types)
    }
    pub(super) fn get_types(&mut self, modules: &ast::Modules) -> DiagnosticResult<Types> {
        let mut types = Types::new();

        for (path, module) in modules.iter() {
            let mut path = path.clone();
            path.remove_extension();

            types
                .modules
                .insert(path.clone(), self.get_module_types(module)?);
        }

        Ok(types)
    }
}
