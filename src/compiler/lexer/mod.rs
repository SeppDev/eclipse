pub(super) mod kind;
pub(super) mod reader;

use std::path::PathBuf;

use crate::common::errors::CompileResult;

use super::context::CompileCtx;

impl CompileCtx {
    pub fn tokenize(&mut self, path: &PathBuf) -> CompileResult<()> {
        let file = self.project_files.read(path).unwrap();
        let mut reader = self.new_reader(&file.body)?;

        reader.advance_if(|c| c == &'c');

        Ok(())
    }
}
