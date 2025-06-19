use std::collections::HashMap;

use common::{constants::FILE_EXTENSION, layout::ast, path::Path};


#[derive(Debug, Default)]
pub struct ASTModule {
    pub imports: Vec<Path>,
    pub body: Vec<ast::Node>,
}

#[derive(Debug, Default)]
pub struct ASTModules {
    pub files: HashMap<Path, ASTModule>,
}

impl ASTModules {
    pub fn entry(&self) -> &ASTModule {
        let path = Path::single("src").join("main").extension(FILE_EXTENSION);
        self.files.get(&path).unwrap()
    }
}
