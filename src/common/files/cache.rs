use std::fs;

use crate::common::errors::CompileResult;

use super::{ProjectFiles, FILE_EXTENSION};

impl ProjectFiles {
    pub fn pre_cache(&mut self) -> CompileResult<()> {
        let dir_path = self.project_path.join("src");
        let dir = fs::read_dir(dir_path)?;

        let mut files = Vec::new();

        loop {}

        for entry in dir {
            let entry = entry?;
        }

        Ok(())
    }
}
