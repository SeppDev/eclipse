use crate::cli::{arguments::Arguments, options::BuildOptions};

pub fn build(arguments: Arguments) {
    let options = BuildOptions::from(arguments);
    println!("Building!")
}
