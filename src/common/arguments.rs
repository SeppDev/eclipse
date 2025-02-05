use std::{env::Args, path::PathBuf};

pub struct Arguments {
    current_args: Args,
    current_dir: PathBuf,
    // exec_path: PathBuf,
}
impl Arguments {
    pub fn new() -> Self {
        let mut current_args = std::env::args();
        let current_dir = std::env::current_dir().unwrap();

        let _ = current_args.next();

        Self {
            current_dir,
            current_args,
        }
    }
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
    pub fn next(&mut self) -> Option<String> {
        self.current_args.next()
    }
}
