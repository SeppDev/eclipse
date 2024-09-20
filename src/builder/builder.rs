use std::path::PathBuf;

use crate::lexer::tokenize;
use crate::lexer::TokensGroup;
use crate::parser::parse;
use crate::read_file;
use crate::BuildError;
use crate::FILE_EXTENSION;

pub fn build(project_path: PathBuf) -> Result<PathBuf, BuildError> {
    let relative_path = PathBuf::from(format!("src/main.{}", FILE_EXTENSION));
    let main_path = project_path.join(&relative_path);

    let source = read_file(&main_path)?;
    
    let tokens = tokenize(source);
    let mut tokengroup = TokensGroup::new(tokens, relative_path);

    let nodes = match parse(&mut tokengroup) {
        Ok(nodes) => nodes,
        Err(error) => return Err(BuildError::CompileError(error))
    };
    println!("{:#?}", nodes);

    // let mut tokensgroup = TokensGroup::new(tokens, relative_path);
    // let nodes = match parse(&mut tokensgroup) {
    //     Ok(nodes) => nodes,
    //     Err(error) => return Err(BuildError::CompileError(error))
    // };
    // println!("{:#?}", nodes);


    todo!()
}
