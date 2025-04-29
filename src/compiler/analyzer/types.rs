use std::collections::HashMap;

use crate::{
    common::position::LocatedAt,
    compiler::{nodes::ast::Identifier, CompilerCtx},
};

pub enum Type {
    Function {},
}
pub struct FunctionTypes {
    parameters: Vec<LocatedAt<(Identifier, LocatedAt<Type>)>>,
    return_type: LocatedAt<Type>,
}

pub struct ModuleTypes {
    pub functions: HashMap<String, Type>,
}

impl CompilerCtx {
    pub(super) fn extract_types(&self) {}
}
