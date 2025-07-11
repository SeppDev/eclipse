#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::path::PathBuf;

    use analyzer::analyze;
    use common::constants::FILE_EXTENSION;
    use context::{CompilerCtx, resolver::MockResolver};

    macro_rules! success_test {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                success($input);
            }
        };
    }
    macro_rules! failed_test {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                failed($input);
            }
        };
    }

    pub fn init(input: &'static str) -> CompilerCtx {
        let mut compiler = CompilerCtx::builder()
            .project_path(PathBuf::new())
            .resolver(MockResolver::new())
            .build();

        let src_path = PathBuf::from("src");
        let mut main_path = src_path.join("main");
        main_path.set_extension(FILE_EXTENSION);
        compiler.write(&main_path, input);

        analyze(&mut compiler, main_path.into());

        compiler
    }

    pub fn success(input: &'static str) {
        let compiler = init(input);
        let result = compiler.diagnostics.has_errors();
        if !result {
            return;
        };

        compiler.diagnostics.display();
        panic!("Expected to not fail\nINPUT:\n{input:#?}")
    }
    pub fn failed(input: &'static str) {
        let compiler = init(input);
        let result = compiler.diagnostics.has_errors();
        if result {
            return;
        };

        panic!("Expected to fail\nINPUT:\n{input:#?}")
    }

    success_test!(main_function, "func main() {}");
    failed_test!(wrong_type, "func main() { var x: i32 = false }");
}
