use std::{collections::HashMap, path::PathBuf};

use crate::{
    lexer::{tokenize, TokensGroup},
    read_file, CompileError, ParseResult, FILE_EXTENSION,
};

use super::{parse, ASTNode};


fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

fn find_path(project_path: &PathBuf, paths: [String; 2]) -> Option<PathBuf> {
    let mut found_path = None;
    for p in paths {
        let file_path = clean_path(project_path.join(&p));
        let full_file_path = project_path.join(&file_path.with_extension(FILE_EXTENSION));

        if full_file_path.exists() {
            found_path = Some(PathBuf::from(p));
            break;
        }
    }
    return found_path;
}

#[derive(Debug)]
pub struct Module {
    pub submodules: HashMap<String, (bool, Module)>,
    pub body: Vec<ASTNode>,
}
impl Module {
    pub fn new(project_path: &PathBuf, relative_path: &PathBuf) -> ParseResult<Self> {
        let file_name = &relative_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let is_module = {
            if relative_path.to_str().unwrap() == "src/mod" {
                return Err(CompileError::new(
                    "file cannot be named 'mod' and be parented to 'src/'".to_string(),
                    0,
                ));
            }
            relative_path.to_str().unwrap() == "src/main" || file_name == "mod"
        };

        let file_path = project_path.join(&relative_path);
        let source = read_file(&file_path.with_extension(FILE_EXTENSION));

        let tokens = tokenize(source);
        let (nodes, imports) = parse(&mut TokensGroup::new(tokens))?;

        let mut submodules = HashMap::new();

        for (export, import) in imports {
            let paths: [String; 2];
            if is_module {
                paths = [import.clone(), format!("{}/mod", import)]
            } else {
                paths = [
                    format!("{}/{}", file_name, import),
                    format!("{}/{}/mod", file_name, import),
                ]
            }

            let parent = relative_path.parent().unwrap();
            let full_relative_path = project_path.join(&parent);

            // TODO better error handling
            let found_path = find_path(&full_relative_path, paths).unwrap();
            let module = Self::new(project_path, &parent.join(&found_path))?;

            submodules.insert(import, (export, module));
        }

        return Ok(Self {
            body: nodes,
            submodules,
        });
    } 
}
