use std::path::PathBuf;

use crate::{common::errors::CompileResult, compiler::context::CompileCtx};

impl CompileCtx {
    pub(super) fn parse_tokens(
        &mut self,
        paths: &mut Vec<PathBuf>,
        current_path: PathBuf,
    ) -> CompileResult<()> {
        println!("{current_path:?}");
        todo!()
    }
}
