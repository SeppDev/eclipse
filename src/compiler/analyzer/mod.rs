use super::{parser::ParsedModules, CompilerCtx};

mod types;

impl CompilerCtx {
    // pub(super) fn check_name_collision(&self) {}
    pub fn analyze(&mut self, modules: &ParsedModules) {
        let types = self.get_types(modules);
        println!("{types:#?}");
    }
}
