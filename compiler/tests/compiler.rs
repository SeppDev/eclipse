#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::path::PathBuf;

    use common::constants::FILE_EXTENSION;
    use compiler::compile;
    use context::{CompilerCtx, files::FileSystemResolver};

    macro_rules! test {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                test($input);
            }
        };
    }

    pub fn test(input: &'static str) {
        #[cfg(unix)]
        let mut temp = PathBuf::from("tmp");
        temp.push("eclipse");

        let entry = CompilerCtx::entry();

        let mut compiler = CompilerCtx::builder().project_path(temp).build();
        compiler.write(&entry, input);

        compile(&mut compiler);
    }

    test!(main_function, "func main() {}");
}
