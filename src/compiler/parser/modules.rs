use crate::{compiler::Path, FILE_EXTENSION};

use super::{ParsedModule, ParsedModules};

impl ParsedModules {
    pub fn entry(&self) -> &ParsedModule {
        let path = Path::single("src").join("main").extension(FILE_EXTENSION);
        self.files.get(&path).unwrap()
    }
}
