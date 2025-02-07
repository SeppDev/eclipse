use std::collections::HashMap;

use super::path::Path;

pub struct ProjectFiles {
    src: Files,
}

#[derive(Default)]
pub struct Files {
    files: HashMap<Path, File>,
}
impl Files {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct File {}
