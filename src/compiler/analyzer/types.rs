use std::collections::HashMap;

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileMessages, CompileResult},
    parser::{Node, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

#[derive(Debug)]
pub struct FileTypes {
    pub imports: HashMap<String, FileTypes>,
    pub functions: HashMap<String, Function>,
    // pub types: HashMap<String, Type>
    pub relative_path: Path,
}
impl FileTypes {
    pub fn get_function(&self, relative_path: &Path, static_path: &Path) -> Option<&Function> {
        

        let mut components = static_path.components();
        let name = components.pop().unwrap();

        if components.len() == 0 {
            return self.functions.get(&name);
        }
        todo!()
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Type>,
    pub return_type: Type,
}

pub fn parse_types(
    compile_messages: &mut CompileMessages,
    name_counter: &mut NameCounter,
    program: &ParsedProgram,
) -> CompileResult<FileTypes> {
    let main = handle_file(compile_messages, name_counter, &program.main)?;

    let mut src = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        relative_path: Path::from("src")
    };
    src.imports.insert(String::from("main"), main);

    return Ok(src);
}

fn handle_file(
    compile_messages: &mut CompileMessages,
    name_counter: &mut NameCounter,
    file: &ParsedFile,
) -> CompileResult<FileTypes> {
    let mut types = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        relative_path: file.relative_path.clone(),
    };

    for (name, import) in &file.imports {
        let file = handle_file(compile_messages, name_counter, import)?;
        let _old = match types.imports.insert(name.clone(), file) {
            Some(old) => old,
            None => continue,
        };
        return Err(());
    }

    for info in &file.body {
        match &info.node {
            Node::Function {
                export,
                name,
                parameters,
                return_type,
                body,
            } => {
                let new_name =
                    if file.relative_path == Path::from("src").join("main") && name.eq("main") {
                        String::from("main")
                    } else {
                        name_counter.increment()
                    };

                types.functions.insert(
                    name.clone(),
                    Function {
                        name: new_name,
                        parameters: {
                            let mut params = Vec::new();
                            for (_, t) in parameters {
                                params.push(t.clone());
                            }
                            params
                        },
                        return_type: return_type.clone(),
                    },
                );
            }
            _ => continue,
        }
    }

    return Ok(types);
}
