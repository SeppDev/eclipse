use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::common::errors::CompileResult;

use super::{File, Files, ProjectFiles, FILE_EXTENSION};

impl Files {
    pub fn pre_cache(&mut self, path: &PathBuf) -> CompileResult<()> {
        let dir_path = path.join("src");
        let dir = fs::read_dir(dir_path)?;

        self.files.clear();

        let mut entries = dir
            .filter_map(|d| d.is_ok().then_some(d.unwrap()))
            .collect::<Vec<DirEntry>>();

        loop {
            let entry = match entries.pop() {
                Some(e) => e,
                None => break,
            };
            let path = entry.path();
            if path.is_dir() {
                entries.append(
                    &mut fs::read_dir(path)?
                        .filter_map(|d| d.is_ok().then_some(d.unwrap()))
                        .collect::<Vec<DirEntry>>(),
                );

                continue;
            }

            if let Some(extension) = path.extension() {
                if extension.to_str().unwrap_or_default() != FILE_EXTENSION {
                    continue;
                }
            }

            let source = fs::read_to_string(&path)?;
            let file = File::from(source);
            self.files.insert(path, file);
        }

        Ok(())
    }
}
