use crate::cli::{arguments::Arguments, options::BuildOptions};

pub fn run(arguments: Arguments) {
    let options = BuildOptions::from(arguments);
    println!("Running!")
}
