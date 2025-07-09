#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use analyzer::analyze;
    use common::constants::FILE_EXTENSION;
    use context::{CompilerCtx, resolver::MockResolver};

    #[test]
    pub fn test() {
        let input = "func main() void {}";

        let project_path = PathBuf::from("test");
        let mut resolver = MockResolver::new();

        let src_path = project_path.join("src");
        let mut main_path = src_path.join("main");
        main_path.set_extension(FILE_EXTENSION);
        resolver.insert(main_path.clone(), input);

        let mut compiler = CompilerCtx::builder()
            .project_path(PathBuf::new())
            .resolver(resolver)
            .build();

        let result = analyze(&mut compiler, main_path.into());
        panic!("{result:#?}");
    }
}
