use std::collections::HashMap;

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult},
    parser::{Node, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

#[derive(Debug)]
pub struct FileTypes {
    functions: HashMap<String, Function>,
    imports: HashMap<String, FileTypes>,
    is_module: bool,
    // pub types: HashMap<String, Type>
    // export: bool,
}
impl FileTypes {
    pub fn get_function(
        &self,
        relative_path: &Path,
        static_path: &Path,
    ) -> CompileResult<Option<&Function>> {
        let mut components = static_path.components();
        let name = components.pop().unwrap();

        let mut new_path = relative_path.clone();
        while components.len() > 0 {
            let key = components.pop().unwrap();
            match &key[..] {
                "root" => new_path.clear(),
                "super" => {
                    new_path.pop();
                }
                _ => new_path.push(key),
            }
        }

        let file = {
            let mut path_components = new_path.components();
            path_components.reverse();
            path_components.pop();

            let mut file = self;
            while path_components.len() > 0 {
                let key = path_components.pop().unwrap();
                let f = match file.imports.get(&key) {
                    Some(f) => f,
                    None => return Ok(None),
                };
                if f.is_module {
                    file = match f.imports.get(&key) {
                        Some(f) => f,
                        None => f,
                    }
                } else {
                    file = f;
                }
            }
            file
        };

        return Ok(file.functions.get(&name));
    }
}

#[derive(Debug)]
pub struct Function {
    pub key: String,
    pub parameters: Vec<Type>,
    pub return_type: Type,
}

pub fn parse_types(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    program: &ParsedProgram,
) -> CompileResult<FileTypes> {
    let main = handle_file(debug, count, &program.main)?;

    let mut src = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        is_module: true,
        // export: true
    };
    src.imports.insert(String::from("main"), main);

    return Ok(src);
}

fn handle_file(
    compile_messages: &mut CompileCtx,
    count: &mut NameCounter,
    file: &ParsedFile,
) -> CompileResult<FileTypes> {
    let mut types = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        is_module: file.is_module,
    };

    for (name, import) in &file.imports {
        let file = handle_file(compile_messages, count, import)?;
        types.imports.insert(name.clone(), file);
    }

    for info in &file.body {
        match &info.node {
            Node::Function {
                export: _,
                name,
                key,
                parameters,
                return_type,
                body: _,
            } => {
                let key = if file.relative_file_path == Path::from("src").join("main")
                    && name.eq("main")
                {
                    String::from("main")
                } else {
                    key.clone()
                };

                types.functions.insert(
                    name.clone(),
                    Function {
                        key,
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
