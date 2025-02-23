use crate::{
    cli::{arguments::Arguments, options::CommandLineOptions},
    compiler::CompilerCtx,
};

pub fn check(arguments: Arguments) {
    let options = CommandLineOptions::from(arguments);
    let mut compiler = CompilerCtx::new(options).unwrap_or_else(|e| e.exit());
    compiler.analyze().unwrap_or_else(|e| e.exit());
}
