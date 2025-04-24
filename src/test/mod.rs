mod lexer;
mod parser;

#[cfg(test)]
mod init {
    use crate::{common::path::Path, compiler::CompilerCtx};

    impl CompilerCtx {
        pub fn test() -> Self {
            Self::builder()
                .status(false)
                .project_path(Path::single("test"))
                .build()
        }
    }
}
