use std::collections::HashMap;

use crate::{
    common::path::Path,
    compiler::{
        common::ast::{Parameter, RawNode, Type}, diagnostics::DiagnosticResult, parser::{ASTModule, ASTModules}, CompilerCtx
    },
};

#[derive(Debug)]
pub struct FunctionType {
    // generics: Vec<String>,
    parameters: Vec<Parameter>,
    return_type: Type,



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

impl CompilerCtx {
    fn get_module_types(&self, module: &ASTModule) -> DiagnosticResult<ModuleTypes> {
        let mut module_types = ModuleTypes::default();
        let functions = &mut module_types.functions;
        // let types = &mut module_types.types;

        for node in &module.body {
            match &node.raw {
                RawNode::Function {
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
    pub(super) fn get_types(&mut self, modules: &ASTModules) -> DiagnosticResult<Types> {
        let mut types = Types::new();

        for (path, module) in &modules.files {
            let mut path = path.clone();
            path.set_extension("");

            types
                .modules
                .insert(path.clone(), self.get_module_types(module)?);
        }

        Ok(types)
    }
}
