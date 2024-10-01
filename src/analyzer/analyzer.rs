
use crate::{analyzer::Types, parser::ASTNode, AnalyzeResult};
use std::{collections::HashMap, path::PathBuf};

pub fn analyze(modules: HashMap<PathBuf, Vec<ASTNode>>) -> AnalyzeResult<()> {
    println!("{:#?}", modules); 
    

    // let types = parse_types(&modules)?;
    // println!("{:#?}", types);
    

    todo!()
}

