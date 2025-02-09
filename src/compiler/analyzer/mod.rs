use crate::common::errors::CompileResult;

use super::context::CompileCtx;

impl CompileCtx {
    pub fn analyze(&mut self) -> CompileResult<()> {
        self.parse()?;

        Ok(())
    }
}
