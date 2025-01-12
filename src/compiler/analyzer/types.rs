use std::collections::HashMap;

use crate::compiler::{
    errors::CompileCtx,
    nodes::{ast, hlir},
    parser::ParsedFile,
    path::Path,
};

#[derive(Debug)]
pub struct FunctionTypes {
    pub key: String,
    pub parameters: Vec<hlir::Type>,
    pub return_type: hlir::Type,
}

#[derive(Debug, Default)]
pub struct ModuleTypes {
    imports: HashMap<String, ModuleTypes>,
    functions: HashMap<String, FunctionTypes>,
}
impl ModuleTypes {
    fn new() -> Self {
        Self::default()
    }
    pub(super) fn get_path(&self, ctx: &mut CompileCtx, path: &Path) -> Option<&FunctionTypes> {
        let mut components = path.components.clone();
        
        let front = components.pop().unwrap();
        let mut file = self.imports.get(&"main".to_string()).unwrap();
        
        components.reverse();
        while let Some(k) = components.pop() {
            file = match file.imports.get(&k) {
                Some(f) => f,
                None => return None
            }
        };
        
        file.functions.get(&front)
    }
}

pub struct ParsedProject {
    pub main: ParsedFile,
    pub std: ParsedFile,
}

pub fn parse_types(ctx: &mut CompileCtx, project: &mut ParsedProject) -> ModuleTypes {
    let mut module = ModuleTypes::new();

    module
        .imports
        .insert("main".to_string(), handle_file(ctx, &mut project.main));
    module
        .imports
        .insert("std".to_string(), handle_file(ctx, &mut project.std));
    return module;
}

fn handle_file(ctx: &mut CompileCtx, file: &mut ParsedFile) -> ModuleTypes {
    let mut module = ModuleTypes::new();
    for (name, file) in &mut file.imports {
        module.imports.insert(name.clone(), handle_file(ctx, file));
    }

    for function in &mut file.functions {
        module.functions.insert(
            function.raw.name.raw.clone(),
            FunctionTypes {
                key: function.raw.key.clone(),
                parameters: function
                    .raw
                    .parameters
                    .iter()
                    .map(|t| t.raw.data_type.raw.convert())
                    .collect(),
                return_type: function.raw.return_type.raw.convert(),
            },
        );
    }

    for layout in file.layouts.drain(..).into_iter() {
        match layout.raw {
            ast::RawLayout::Struct { name, fields } => continue,
            ast::RawLayout::Enum { name, fields } => continue,
        }
    }

    return module;
}
