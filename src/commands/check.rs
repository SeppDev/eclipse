use crate::{
    cli::{arguments::Arguments, options::CommandLineOptions},
    compiler::CompilerCtx,
};

pub fn check(arguments: Arguments) {
    let compiler: CompilerCtx = CommandLineOptions::from(arguments).into();
    compiler.analyze();
}

impl Into<CompilerCtx> for CommandLineOptions {
    fn into(self) -> CompilerCtx {
        CompilerCtx::builder().status(self.status).build()
    }
}
