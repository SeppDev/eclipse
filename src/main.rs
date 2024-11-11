use std::path::PathBuf;

mod compiler;

fn main() {
    let mut project_dir = std::env::current_dir().unwrap();
    let mut args = std::env::args();
    args.next();
    
    match args.next() {
        Some(path) => project_dir = PathBuf::from(path),
        None => {}
    }

    compiler::build(project_dir);
}