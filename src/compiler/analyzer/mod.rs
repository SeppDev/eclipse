use super::{
    parser::{ParsedModule, ParsedModules},
    CompilerCtx,
};

mod types;

impl CompilerCtx {
    pub fn analyze(&mut self, modules: ParsedModules) {
        let entry = modules.entry();
        self.log(format!("{entry:#?}"));
    }
}
