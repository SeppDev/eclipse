use super::{ParsedModule, ParsedModules};
use crate::{compiler::Path, FILE_EXTENSION};

impl ParsedModules {
    pub fn _entry(&self) -> &ParsedModule {
        let path = Path::single("src").join("main").extension(FILE_EXTENSION);
        self.files.get(&path).unwrap()
    }
}
