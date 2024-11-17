use std::collections::HashMap;

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileMessages, CompileResult},
    parser::ParsedFile,
    program::ParsedProgram, types::Type,
};

#[derive(Debug, Default)]
pub struct FileTypes {
    imports: HashMap<String, FileTypes>,
    functions: HashMap<String, Function>
}
impl FileTypes {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Type>,
    pub return_type: Type
}

pub fn parse_types(
    compile_messages: &mut CompileMessages,
    name_counter: &mut NameCounter,
    program: &ParsedProgram,
) -> CompileResult<FileTypes> {

    handle_file(&program.main)?;
    todo!()
}

fn handle_file(file: &ParsedFile) -> CompileResult<FileTypes> {
    let mut file = FileTypes::new();

    
    
    return Ok(file);
}
