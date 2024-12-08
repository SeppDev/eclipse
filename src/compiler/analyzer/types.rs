use std::{collections::HashMap, os::linux::raw::stat};

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult, Location},
    parser::{Node, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::{BaseType, Type},
};

#[derive(Debug)]
pub struct CustomEnum {
    pub fields: Vec<String>,
}

#[derive(Debug)]
pub struct CustomStruct {}

#[derive(Debug)]
pub enum CustomTypes {
    Enum(CustomEnum),
    Struct(CustomStruct),
}

#[derive(Debug)]
pub struct ProgramTypes {
    pub src: FileTypes,
    pub std: FileTypes,
}

impl ProgramTypes {
    pub fn get_function(&self, relative_path: &Path, static_path: &Path) -> Option<&Function> {
        let mut components = static_path.components();
        let mut relative_components = relative_path.components();

        let first_relative = relative_components.remove(0);
        let mut file = if first_relative == "std" {
            &self.std
        } else if components.first().unwrap() == "std" {
            components.remove(0);
            relative_components.clear();
            &self.std
        } else {
            &self.src
        };

        let mut find_path: Vec<String> = Vec::new();
        let name = components.pop().unwrap();

        find_path.extend_from_slice(&relative_components);
        find_path.extend_from_slice(&components);

        for component in find_path {
            file = match file.imports.get(&component) {
                Some(f) => f,
                None => return None,
            };
        }

        return file.functions.get(&name);
    }
}

#[derive(Debug)]
pub struct FileTypes {
    functions: HashMap<String, Function>,
    types: HashMap<String, CustomTypes>,
    imports: HashMap<String, FileTypes>,
    // pub types: HashMap<String, Type>
    // export: bool,
}
impl FileTypes {
    // pub fn get_function(&self, relative_path: &Path, static_path: &mut Path) -> Option<&Function> {
    //     let mut components = static_path.components();
    //     let name = components.pop().unwrap();

    //     let mut new_path = relative_path.clone();
    //     while components.len() > 0 {
    //         let key = components.pop().unwrap();
    //         match &key[..] {
    //             "root" => new_path.clear(),
    //             "super" => {
    //                 new_path.pop();
    //             }
    //             _ => new_path.push(key),
    //         }
    //     }

    //     let file = {
    //         let mut path_components = new_path.components();
    //         path_components.reverse();
    //         path_components.pop();

    //         let mut file = self;
    //         while path_components.len() > 0 {
    //             let key = path_components.pop().unwrap();
    //             let f = match file.imports.get(&key) {
    //                 Some(f) => f,
    //                 None => return None,
    //             };
    //             if f.is_module {
    //                 file = match f.imports.get(&key) {
    //                     Some(f) => f,
    //                     None => f,
    //                 }
    //             } else {
    //                 file = f;
    //             }
    //         }
    //         file
    //     };

    //     return file.functions.get(&name);
    // }
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
) -> CompileResult<ProgramTypes> {
    let main = handle_file(debug, count, &program.main)?;
    let mut standard = handle_file(debug, count, &program.standard)?;

    let mut src = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        types: HashMap::new(),
    };

    let io = standard.imports.get_mut("io").unwrap();

    io.functions.insert(
        "print".to_string(),
        Function {
            key: "print".to_string(),
            parameters: vec![(Type::new(BaseType::Int(32)))],
            return_type: Type::void(),
        },
    );

    let thread = standard.imports.get_mut("thread").unwrap();

    thread.functions.insert(
        "sleep".to_string(),
        Function {
            key: "sleep".to_string(),
            parameters: vec![(Type::new(BaseType::Int(32)))],
            return_type: Type::new(BaseType::Int(32)),
        },
    );

    thread.functions.insert(
        "usleep".to_string(),
        Function {
            key: "usleep".to_string(),
            parameters: vec![(Type::new(BaseType::Int(32)))],
            return_type: Type::new(BaseType::Int(32)),
        },
    );

    src.imports.insert(String::from("main"), main);

    return Ok(ProgramTypes { src, std: standard });
}

fn handle_file(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    file: &ParsedFile,
) -> CompileResult<FileTypes> {
    let mut types = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        types: HashMap::new(),
    };

    for (name, import) in &file.imports {
        let file = handle_file(debug, count, import)?;
        if types.imports.insert(name.clone(), file).is_some() {
            debug.error(Location::void(), format!("'{}' is already imported", name));
        };
    }

    for info in &file.body {
        match &info.node {
            Node::Enum { name, fields } => {
                let custom_enum = CustomEnum {
                    fields: fields.clone(),
                };

                types
                    .types
                    .insert(name.clone(), CustomTypes::Enum(custom_enum));
            }
            Node::Function {
                export: _,
                name,
                key,
                parameters,
                return_type,
                body: _,
            } => {
                let is_main_function =
                    file.relative_file_path == Path::from("src").join("main") && name.eq("main");
                let key = if is_main_function {
                    String::from("main")
                } else {
                    key.clone()
                };

                types.functions.insert(
                    name.clone(),
                    Function {
                        key,
                        parameters: parameters
                            .iter()
                            .map(|parameter| parameter.data_type.clone())
                            .collect::<Vec<Type>>(),
                        return_type: return_type.clone(),
                    },
                );
            }
            _ => continue,
        }
    }

    return Ok(types);
}
