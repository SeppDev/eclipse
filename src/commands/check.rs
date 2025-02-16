use crate::cli::{arguments::Arguments, options::BuildOptions};

pub fn check(arguments: Arguments) {
    let options = BuildOptions::from(arguments);
    println!("Checking!")
}