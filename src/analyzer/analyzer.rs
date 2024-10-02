
use crate::{analyzer::parse_datastructures, parser::{ASTNode, Modules}, AnalyzeResult};
use std::{collections::HashMap, path::PathBuf};


pub fn analyze(modules: Modules) -> AnalyzeResult<()> {
    println!("{:#?}", modules); 
    let types = parse_datastructures(&modules)?;


    // let types = parse_types(&modules)?;
    // println!("{:#?}", types);
    

    todo!()
}

