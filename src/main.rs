use compiler::benchmark;

mod compiler;

fn main() {
    let project_dir = std::env::current_dir().unwrap();
    let (_, duration) = benchmark(|| compiler::build(project_dir));
    println!("Compiling took: {:?}", duration);  
}