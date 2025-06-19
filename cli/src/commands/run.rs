use crate::{
    cli::{arguments::Arguments, options::CommandLineOptions},
    compiler::CompilerCtx,
};

pub fn run(arguments: Arguments) {
    let _compiler: CompilerCtx = CommandLineOptions::from(arguments).into();
    println!("Running!")
}
