use std::collections::HashMap;

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileMessages, CompileResult},
    parser::{Node, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

#[derive(Debug, Default)]
pub struct FileTypes {
    pub functions: HashMap<String, Function>,
}
impl FileTypes {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<NodeInfo>,
}

pub fn parse_types(
    compile_messages: &mut CompileMessages,
    name_counter: &mut NameCounter,
    program: ParsedProgram,
) -> CompileResult<FileTypes> {
    let main = handle_file(compile_messages, name_counter, program.main)?;
    println!("{:#?}", main);
    todo!()
}

#[allow(unused)]
fn handle_file(
    compile_messages: &mut CompileMessages,
    name_counter: &mut NameCounter,
    file: ParsedFile,
) -> CompileResult<FileTypes> {
    let mut types = FileTypes::new();

    for info in file.body {
        match info.node {
            Node::Function {
                export,
                name,
                parameters,
                return_type,
                body,
            } => {
                let new_name = if file.relative_path == Path::from("src").join("main") {
                    String::from("main")
                } else {
                    name_counter.increment()
                };
                
                types.functions.insert(
                    name,
                    Function {
                        name: name_counter.increment(),
                        parameters,
                        return_type,
                        body,
                    },
                );
            }
            _ => continue,
        }
    }

    return Ok(types);
}
