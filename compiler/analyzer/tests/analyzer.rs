#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::path::PathBuf;

    use analyzer::analyze;
    use common::constants::FILE_EXTENSION;
    use context::{CompilerCtx, files::MockResolver};
    use resolver::resolve_modules;

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

        let entry = main_path;
        let collection = resolve_modules(&mut compiler, &entry);
        let hlir = analyze(&mut compiler, collection, &entry);

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

    success_test!(main_function_empty, "func main() {}");
    success_test!(main_function_return, "func main() { return }");
    success_test!(
        main_function_return_with_expresion,
        "func main() { return 0 }"
    );
    success_test!(type_inheritance, "func main() { var x: bool = false }");
    success_test!(variable_type_boolean, "func main() { var x: bool = false }");
    success_test!(variable_type_integer, "func main() { var x: i32 = 42 }");

    failed_test!(wrong_return_type, "func main() bool { return 0 }");
    failed_test!(
        wrong_return_type_missing_expression,
        "func main() bool { return }"
    );
    failed_test!(void_return_type_expression, "func main() { return 0 }");
    failed_test!(wrong_type, "func main() { var x: i32 = false }");
    failed_test!(function_as_expression, "func main() func test()");
}
