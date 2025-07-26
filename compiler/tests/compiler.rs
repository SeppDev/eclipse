#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use compiler::compile;
    use context::CompilerCtx;

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
        #[cfg(unix)]
        let mut temp = PathBuf::from("/tmp");

        #[cfg(windows)]
        let mut temp = PathBuf::from("\\%TEMP%");

        temp.push("eclipse");

        let entry = CompilerCtx::entry();

        let mut compiler = CompilerCtx::builder().project_path(temp).build();
        compiler.write(&entry, input);

        compile(&mut compiler);

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

    // success_test!(main_function, "func main() {}");
    success_test!(main_function_return_zero, "func main() i32 { return 0 }");
}
