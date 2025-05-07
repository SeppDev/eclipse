use std::collections::HashMap;

use crate::{
    common::path::Path,
    compiler::{
        diagnostics::DiagnosticResult,
        parser::{ParsedModule, ParsedModules},
        CompilerCtx,
    },
};

#[derive(Debug)]
pub enum Type {
    Void,
    Int(usize),
    Generic(String),
}
// impl Into<Type> for &nodes::ast::Type {
//     fn into(self) -> Type {
//         todo!()
//     }
// }

#[derive(Debug)]
pub struct FunctionType {
    // generics: Vec<String>,
    // parameters: Vec<LocatedAt<(Identifier, LocatedAt<Type>)>>,
    // return_type: LocatedAt<Type>,
}

#[derive(Debug, Default)]
pub struct ModuleTypes {
    pub types: HashMap<String, Type>,
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
    fn get_module_types(&self, module: &ParsedModule) -> DiagnosticResult<ModuleTypes> {
        let mut module_types = ModuleTypes::default();
        let functions = &mut module_types.functions;
        let types = &mut module_types.types;

        for node in &module.body {
            match &node.raw {
                // RawNode::Function {
                //     name,
                //     parameters,
                //     return_type,
                //     ..
                // } => {
                //     let parameters = parameters.iter().map(|t| );
                //     let return_type = return_type.raw.into();
                //     functions.insert(
                //         name.raw.clone(),
                //         FunctionType {
                //             parameters,
                //             return_type,
                //         })},
                _ => todo!(),
            };
        }

        Ok(module_types)
    }
    pub(super) fn get_types(&mut self, modules: &ParsedModules) -> DiagnosticResult<Types> {
        let mut types = Types::new();

        for (path, module) in &modules.files {
            types
                .modules
                .insert(path.clone(), self.get_module_types(module)?);
        }

        Ok(types)
    }
}
