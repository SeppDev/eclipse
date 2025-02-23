use std::{fs, path::PathBuf};

use crate::{cli::arguments::Arguments, common::exit::exit};

pub fn new(mut arguments: Arguments) {
    let (main, config) = if cfg!(any(unix, windows)) {
        (
            include_str!("../static/main"),
            include_str!("../static/config.toml"),
        )
    } else {
        exit("Platform is not supported")
    };

    let name = arguments.expect_argument(None).into_value();
    let directory = match arguments.next_argument() {
        Some(value) => PathBuf::from(value.into_value()),
        None => arguments.current_dir().clone(),
    };
    let project_path = directory.join(&name);
    if project_path.exists() {
        exit(format!(
            "There is already a project named: '{name}' in this directory"
        ));
    }

    let main_path = project_path.join("src/main.ecl");
    fs::create_dir_all(project_path.join("src")).unwrap();

    fs::write(main_path, main).unwrap();
    fs::write(project_path.join("config.toml"), config).unwrap();
}