use std::{collections::HashMap, path::PathBuf};

use crate::{
    lexer::{tokenize, TokensGroup},
    read_file, CompileError, ParseResult, FILE_EXTENSION,
};

use super::{parse, ASTNode, Node, Path};

fn clean_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(path.to_string_lossy().replace("\\", "/"));
}

fn find_path(
    project_path: &PathBuf,
    paths: [String; 2],
    line: usize,
) -> ParseResult<PathBuf> {
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
            return Err(CompileError::new(
                format!("Import path failed: {:#?}", paths),
                line,
            ));
        }
    };
}

fn parse_tokens(project_path: &PathBuf, relative_path: &PathBuf, nodes: Vec<ASTNode>) -> ParseResult<Vec<ASTNode>> {
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
        return Err(CompileError::new(
            String::from("Cannot have a mod in the 'src' directory"),
            0,
        ));
    }

    for node in &nodes {
        let name = match &node.node {
            Node::Import(public, name) => name,
            _ => continue,
        };
        let paths: [String; 2];

        if is_module {
            paths = [name.clone(), format!("{}/mod", name)]
        } else {
            paths = [
                format!("{}/{}", file_name, name),
                format!("{}/{}/mod", file_name, name),
            ]
        }
        let found_path= find_path(project_path, paths, node.lines.start);

        // let nodes = parse_tokens(project_path, &found_path, modules)?;
        // modules.insert(Path::normalize(&found_path), nodes);
    }

    return Ok(nodes);
}

pub struct Module {
    pub imports: HashMap<(bool, String), Module>,
}
impl Module {
    pub fn new(project_path: &PathBuf, relative_path: &PathBuf) -> ParseResult<Self> {
        let mut imports = HashMap::new();

        let file_path = project_path.join(&relative_path);
        let source = read_file(&file_path.with_extension(FILE_EXTENSION));

        let tokens = tokenize(source);
        let mut tokengroup = TokensGroup::new(tokens);

        let nodes = parse(&mut tokengroup)?;

        return Ok(Self { imports });
    }
}
