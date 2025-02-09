use std::path::PathBuf;

use crate::common::{errors::CompileResult, files::FILE_EXTENSION};

use super::context::CompileCtx;

impl CompileCtx {
    pub fn analyze(&mut self) -> CompileResult<()> {
        let mut main_path = PathBuf::from("src/main");
        main_path.set_extension(FILE_EXTENSION);
        let tokens = self.tokenize(&main_path)?;
        // println!("{tokens:#?}");

        Ok(())
    }
}
