use std::{fs, path::PathBuf};

use crate::{FILE_EXTENSION, cli::arguments::Arguments};

pub fn init(_arguments: Arguments) {
    todo!()
}

pub(super) fn init_project(project_path: PathBuf, _name: String) {
    let (main, config) = {
        (
            include_str!("../static/main"),
            include_str!("../static/config.toml"),
        )
    };

    let source_dir = project_path.join("src");
    fs::create_dir_all(&source_dir).unwrap();

    let mut main_path = source_dir.join("main");
    main_path.set_extension(FILE_EXTENSION);

    fs::write(main_path, main).unwrap();
    fs::write(project_path.join("eclipse.toml"), config).unwrap();
}
