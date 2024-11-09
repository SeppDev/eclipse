
mod compiler;

fn main() {
    let project_dir = std::env::current_dir().unwrap();
    compiler::build(project_dir);
    
}