use std::path::PathBuf;

use crate::{cli::arguments::Arguments, common::exit::exit};

pub fn new(mut arguments: Arguments) {
    let name = arguments.expect_argument(None).into_value();
    let dir = match arguments.next_argument() {
        Some(value) => PathBuf::from(value.into_value()),
        None => arguments.current_dir().clone(),
    };
    let project_path = dir.join(&name);
    if project_path.exists() {
        exit(format!(
            "There is already a project named: '{name}' in this directory"
        ));
    }

    super::init_project(project_path, name);
}

