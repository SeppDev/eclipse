use std::{collections::HashMap, path::PathBuf};

use crate::{
    builder::Module, lexer::tokenize, read_file, BuildError, BuildProblem, CompileError,
    FILE_EXTENSION,
};

use super::{parse, Node};

fn get_path(project_path: &PathBuf, paths: [PathBuf; 2]) -> Result<PathBuf, ()> {
    for path in paths {
        if project_path.join(&path).exists() {
            return Ok(path);
        }
    }
    return Err(());
}
#[derive(Debug)]
pub struct Program {
    // main: Module,
    pub project_path: PathBuf,
    pub modules: HashMap<PathBuf, Module>,
}
impl Program {
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            project_path,
            modules: HashMap::new(),
        }
    }
    pub fn parse(&mut self, relative_path: PathBuf) -> Result<(), CompileError> {
        let full_path = self.project_path.join(&relative_path);
        let file_name = relative_path.file_stem().unwrap().to_str().unwrap();
        let is_module_root = match file_name {
            "main" => true,
            "mod" => true,
            _ => false,
        };

        let source = match read_file(&full_path) {
            Ok(source) => source,
            Err(error) => return Err(error),
        };
        let mut tokens = match tokenize(source, relative_path.clone()) {
            Ok(tokens) => tokens,
            Err((message, reader)) => {
                return Err(CompileError::BuildProblem(BuildProblem::new(
                    BuildError::Tokenize(message),
                    relative_path,
                    reader.line,
                )))
            }
        };
        let nodes = match parse(&mut tokens) {
            Ok(nodes) => nodes,
            Err(error) => return Err(error),
        };

        for ast in &nodes {
            let node = &ast.node;
            match node {
                Node::Import(module, _) => {
                    let mut parent =
                        String::from(relative_path.parent().unwrap().to_str().unwrap());
                    parent.push('/');

                    let file_paths: [PathBuf; 2];

                    if is_module_root == true {
                        let file =
                            PathBuf::from(&parent).join(format!("{}.{}", module, FILE_EXTENSION));

                        let module = PathBuf::from(&parent)
                            .join(format!("{}/mod.{}", module, FILE_EXTENSION));

                        file_paths = [file, module];
                    } else {
                        let file = PathBuf::from(&parent)
                            .join(format!("{}/{}.{}", file_name, module, FILE_EXTENSION));

                        let module = PathBuf::from(&parent)
                            .join(format!("{}/mod.{}", module, FILE_EXTENSION));

                        file_paths = [file, module];
                    }

                    let path = match get_path(&self.project_path, file_paths.clone()) {
                        Ok(path) => path,
                        Err(_) => {
                            return Err(CompileError::BuildProblem(BuildProblem::new(
                                BuildError::CannotFindModules(file_paths),
                                relative_path,
                                ast.line,
                            )))
                        }
                    };

                    match self.parse(path.clone()) {
                        Ok(()) => break,
                        Err(error) => return Err(error),
                    };
                }
                _ => continue,
            }
        }
        self.modules.insert(relative_path, Module { body: nodes });

        Ok(())
    }
}
