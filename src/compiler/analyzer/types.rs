use std::collections::HashMap;

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult, Location},
    parser::{Node, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::{BaseType, Type},
};

#[derive(Debug)]
struct FieldMap<Key: Eq + PartialEq, Value> {
    fields: Vec<(Key, Value)>,
}
impl<Key: Eq + PartialEq, Value> FieldMap<Key, Value> {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }
    pub fn find(&self, key: &Key) -> Option<usize> {
        for (index, (k, _)) in self.fields.iter().enumerate() {
            if key != k {
                continue;
            }
            return Some(index);
        }
        return None;
    }
    pub fn insert(&mut self, key: Key, value: Value) -> Result<(), Value> {
        let removed = match self.find(&key) {
            Some(index) => Some(self.fields.swap_remove(index)),
            None => None,
        };

        self.fields.push((key, value));

        return match removed {
            Some((_, value)) => Err(value),
            None => Ok(()),
        };
    }
}

#[derive(Debug)]
pub struct CustomEnum {
    offsets: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct CustomStruct {
    pub size: usize,
    fields: FieldMap<String, Type>,
    offsets: HashMap<String, usize>,
}

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
    program: &mut ParsedProgram,
) -> CompileResult<ProgramTypes> {
    let main = handle_file(debug, count, &mut program.main)?;
    let mut standard = handle_file(debug, count, &mut program.standard)?;

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
    file: &mut ParsedFile,
) -> CompileResult<FileTypes> {
    let mut types = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        types: HashMap::new(),
    };

    for (name, import) in &mut file.imports {
        let file = handle_file(debug, count, import)?;
        if types.imports.insert(name.clone(), file).is_some() {
            debug.error(Location::void(), format!("'{}' is already imported", name));
        };
    }

    use std::mem::take;

    for info in &mut file.body {
        match &mut info.node {
            // Node::Enum { name, fields } => {
            //     let name = take(name);
            //     let fields = take(fields);

            //     let custom_enum = CustomEnum {
            //         fields,
            //     };

            //     types
            //         .types
            //         .insert(name.clone(), CustomTypes::Enum(custom_enum));
            // },
            Node::Struct { name, fields } => {
                let name = take(name);
                let vec_fields = take(fields);

                let mut fields = FieldMap::new();
                let mut offsets = HashMap::new();

                let mut offset = 0;
                for (key, data_type) in vec_fields {
                    let size = data_type.bytes();
                    let result = fields.insert(key.clone(), data_type);

                    if result.is_err() {
                        debug.error(info.location.clone(), format!("Duplicate key: {key}"));
                        break;
                    }

                    offsets.insert(key, offset);
                    offset += size;
                }

                let custom_struct = CustomStruct {
                    fields,
                    offsets,
                    size: offset,
                };

                types.types.insert(name, CustomTypes::Struct(custom_struct));
            }
            _ => continue,
        }
    }

    for info in &file.body {
        match &info.node {
            Node::Function {
                name,
                key,
                parameters,
                return_type,
                body: _,
            } => {
                let is_main_function =
                    file.relative_file_path == Path::from("src").join("main") && name == "main";
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
                            .into_iter()
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
