use std::{collections::HashMap, path::PathBuf};

use crate::{
    lexer::{tokenize, TokensGroup},
    read_file, BuildError, CompileError, FILE_EXTENSION,
};

use super::{parse, ASTNode, Node};

type Modules = HashMap<PathBuf, Vec<ASTNode>>;

fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

fn parse_module(
    project_path: &PathBuf,
    relative_path: &PathBuf,
    modules: &mut Modules,
) -> Result<Vec<ASTNode>, BuildError> {
    let file_path = project_path.join(&relative_path);
    let source = read_file(&file_path.with_extension(FILE_EXTENSION))?;

    let tokens = tokenize(source);
    let mut tokengroup = TokensGroup::new(tokens);

    let nodes = match parse(&mut tokengroup) {
        Ok(n) => n,
        Err(err) => return Err(BuildError::CompileError(err)),
    };

    let main_path = PathBuf::from("src/main");
    let file_name = relative_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let is_module = file_name == "mod" || relative_path == &main_path;
    let parent = relative_path.parent().unwrap();
    if file_name == "mod" && parent == PathBuf::from("src/") {
        return Err(BuildError::CompileError(CompileError::new(
            String::from("Cannot have a mod in the 'src' directory"),
            0,
        )));
    }

    for node in &nodes {
        let name = match &node.node {
            Node::Import(name) => name,
            _ => continue,
        };
        let paths: [String; 2];

        if is_module {
            paths = [format!("{}", name), format!("{}/mod", name)]
        } else {
            paths = [
                format!("{}/{}", file_name, name),
                format!("{}/{}/mod", file_name, name),
            ]
        }

        let mut found_path: Option<PathBuf> = None;
        for p in &paths {
            let path = clean_path(parent.join(p));
            if project_path
                .join(&path.with_extension(FILE_EXTENSION))
                .exists()
            {
                found_path = Some(path);
                break;
            }
        }
        let found_path = match found_path {
            Some(p) => p,
            None => {
                return Err(BuildError::CompileError(CompileError::new(
                    format!("Import path failed: {:#?}", paths),
                    0,
                )))
            }
        };
        let nodes = parse_module(project_path, &found_path, modules)?;
        modules.insert(found_path, nodes);
    }

    return Ok(nodes);
}

pub fn parse_modules(project_path: PathBuf) -> Result<Modules, BuildError> {
    let main_path = PathBuf::from("src/main");
    let mut modules = HashMap::new();
    let nodes = parse_module(&project_path, &main_path, &mut modules)?;
    modules.insert(main_path, nodes);

    return Ok(modules);
}
