use super::Program;
use crate::{parser::ASTNode, AnalyzeResult};
use std::{collections::HashMap, path::PathBuf};

pub fn analyze(modules: HashMap<PathBuf, Vec<ASTNode>>) -> AnalyzeResult<Program> {
    println!("{:#?}", modules); 

    // let types = parse_types(&modules)?;
    // println!("{:#?}", types);
    

    todo!()
}

