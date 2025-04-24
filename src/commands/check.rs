use crate::{
    cli::{arguments::Arguments, options::CommandLineOptions},
    compiler::CompilerCtx,
};

pub fn check(arguments: Arguments) {
    let mut compiler: CompilerCtx = CommandLineOptions::from(arguments).into();
    let nodes = compiler.parse().unwrap();
    drop(compiler);

    println!("{nodes:#?}");
}

impl Into<CompilerCtx> for CommandLineOptions {
    fn into(self) -> CompilerCtx {
        CompilerCtx::builder()
            .project_path(self.active_path.into())
            .status(self.status)
            .build()
    }
}
