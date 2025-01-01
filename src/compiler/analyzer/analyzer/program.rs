use crate::compiler::{
    analyzer::ProgramTypes, codegen::CodeGen, errors::CompileCtx, path::Path, types::Type,
};

use super::FunctionCtx;

#[derive(Debug)]
pub struct ProgramCtx<'a> {
    pub debug: &'a mut CompileCtx,
    pub codegen: CodeGen,
    pub types: &'a ProgramTypes,
    pub namespaces: Vec<Path>,
}
impl<'a> ProgramCtx<'a> {
    pub fn new(debug: &'a mut CompileCtx, types: &'a ProgramTypes) -> Self {
        Self {
            debug,
            types,
            codegen: CodeGen::new(),
            namespaces: Vec::new(),
        }
    }
    pub fn get_type(&self, function: &FunctionCtx, path: &Path) -> Option<&Type> {
        self.types
            .get_type(function.relative_file_path, &path, &self.namespaces)
    }
}
