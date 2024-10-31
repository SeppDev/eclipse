use std::{collections::HashMap, path::PathBuf};

use crate::{
    analyzer::{IRProgram, RandomString},
    execute,
};

use super::llvm;

pub fn codegen(
    project_dir: &PathBuf,
    program: IRProgram,
    mode: Mode,
    random_string: RandomString,
) -> PathBuf {
    use std::fs;

    let mut builder = Builder::new(mode.clone(), random_string);

    let mut build_dir = project_dir.clone();
    build_dir.push("build");

    // if build_dir.exists() {
    // fs::remove_dir_all(&build_dir).unwrap();
    // }
    fs::create_dir_all(&build_dir).unwrap();

    let mut main_dir = build_dir.clone();
    main_dir.push("main.ll");

    let string_path = main_dir.to_str().unwrap().to_string();
    let build_dir_path = build_dir.to_str().unwrap().to_string();

    match mode {
        Mode::LLVM => {
            llvm::generate(program, &mut builder);

            fs::write(&main_dir, builder.build()).unwrap();

            let output = match execute(format!(
                "clang -O3 {} -o {}/build.exe",
                &string_path, build_dir_path
            )) {
                Ok(out) => out,
                Err(error) => panic!("{}", error),
            };

            if output.status.success() == false {
                println!("LLVM {}", output.status);
                panic!("{}", String::from_utf8(output.stderr).unwrap());
            }
        }
    }

    return build_dir.join("build.exe");
}

#[derive(Debug, Clone)]
pub enum Mode {
    LLVM,
    // MLIR,
    // CraneLift,
}

#[derive(Debug)]
struct Body {
    pub body: String,
}
impl Body {
    pub fn new() -> Self {
        Self {
            body: String::new(),
        }
    }
    pub fn pushln(&mut self, line: String) {
        self.body.push_str(line.as_str());
        self.body.push('\n');
    }
    pub fn pushln_str(&mut self, line: &str) {
        self.body.push_str(line);
        self.body.push('\n');
    }
}

#[derive(Debug)]
pub struct Builder {
    pub random: RandomString,
    mode: Mode,
    constants: HashMap<String, String>,
    body: String,
}
impl Builder {
    pub fn new(mode: Mode, random: RandomString) -> Self {
        Self {
            mode,
            random,
            constants: HashMap::new(),
            body: String::new(),
        }
    }
    pub fn build(&self) -> String {
        let mut body = Body::new();
        match &self.mode {
            Mode::LLVM => build_llvm(self, &mut body),
        }
        body.pushln(self.body.clone());
        return body.body;
    }
    pub fn contstant_string(&mut self, string: String) -> String {
        let name = format!(".str.{}", self.constants.len());
        self.constants.insert(name.clone(), string);
        return name;
    }

    pub fn next_line(&mut self) {
        self.body.push('\n');
    }

    pub fn push(&mut self, line: String) {
        self.body.push_str(line.as_str());
    }
    pub fn push_str(&mut self, line: &str) {
        self.body.push_str(line);
    }

    pub fn pushln(&mut self, line: String) {
        self.body.push_str(line.as_str());
        self.body.push('\n');
    }
    pub fn pushln_str(&mut self, line: &str) {
        self.body.push_str(line);
        self.body.push('\n');
    }
}

fn build_llvm(builder: &Builder, body: &mut Body) {
    body.pushln_str("target triple = \"x86_64-pc-windows-unkown\"\n");
    body.pushln_str("declare i32 @printf(i8*, ...)");
    body.pushln_str("@.str = private constant [4 x i8] c\"%d\\0A\\00\"\n");

    body.pushln_str("define void @print(i32 %a) local_unnamed_addr #0 {\nentry:");
    body.pushln_str("\t%str_ptr = getelementptr [4 x i8], [4 x i8]* @.str, i32 0, i32 0");
    // body.pushln_str("\t%.a = load i32, i32* %a");
    body.pushln_str("\tcall i32 @printf(i8* %str_ptr, i32 %a)");
    body.pushln_str("\tret void");
    body.pushln_str("}");

    for (name, value) in &builder.constants {
        body.pushln(format!(
            "@{} = private constant [{} x i8] c\"{:?}\"",
            name,
            value.len(),
            value
        ));
    }
}
