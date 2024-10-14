use std::{collections::HashMap, path::PathBuf};

use crate::{
    lexer::{tokenize, TokensGroup},
    read_file, CompileError, ParseResult, FILE_EXTENSION,
};

use super::{parse, ASTNode};

#[allow(unused)]
fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

#[allow(unused)]
fn find_path(project_path: &PathBuf, paths: [String; 2]) -> Option<PathBuf> {
    let mut found_path: Option<PathBuf> = None;
    for p in paths {
        let path = clean_path(project_path.join(p));
        if project_path
            .join(&path.with_extension(FILE_EXTENSION))
            .exists()
        {
            found_path = Some(path);
            break;
        }
    }
    return found_path;
}

// fn parse_tokens(
//     project_path: &PathBuf,
//     relative_path: &PathBuf,
//     nodes: Vec<ASTNode>,
// ) -> ParseResult<Vec<ASTNode>> {
//     let main_path = PathBuf::from("src/main");
//     let file_name = relative_path
//         .file_stem()
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string();

//     let is_module = file_name == "mod" || relative_path == &main_path;
//     let parent = relative_path.parent().unwrap();
//     if file_name == "mod" && parent == PathBuf::from("src/") {
//         return Err(CompileError::new(
//             String::from("Cannot have a mod in the 'src' directory"),
//             0,
//         ));
//     }

//     for node in &nodes {
//         let paths: [String; 2];

//         if is_module {
//             paths = [name.clone(), format!("{}/mod", name)]
//         } else {
//             paths = [
//                 format!("{}/{}", file_name, name),
//                 format!("{}/{}/mod", file_name, name),
//             ]
//         }
//         let found_path = match find_path(project_path, &paths) {
//             Some(p) => p,
//             None => {
//                 return Err(CompileError::new(
//                     format!("Import path failed: {:#?}", paths),
//                     node.lines.start,
//                 ))
//             }
//         };

//         // let nodes = parse_tokens(project_path, &found_path, modules)?;
//         // modules.insert(Path::normalize(&found_path), nodes);
//     }

//     return Ok(nodes);
// }

#[derive(Debug)]
pub struct Module {
    pub submodules: HashMap<String, (bool, Module)>,
    pub body: Vec<ASTNode>,
}
#[allow(unused)]
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
                paths = [file_name.clone(), format!("{}/mod", file_name)]
            } else {
                paths = [
                    format!("{}/{}", import, file_name),
                    format!("{}/{}/mod", import, file_name),
                ]
            }

            let path = find_path(project_path, paths);
            todo!("{:?}", path);
        }

        return Ok(Self {
            body: nodes,
            submodules,
        });
    }
}
