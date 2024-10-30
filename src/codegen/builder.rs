use std::{collections::HashMap, path::PathBuf};

use crate::{analyzer::IRProgram, execute};

use super::llvm;

pub fn codegen(project_dir: &PathBuf, program: IRProgram, mode: Mode) -> PathBuf {
    use std::fs;

    let mut builder = Builder::new(mode.clone());

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
            

            match execute(format!("clang -O3 {} -o {}/build.exe", &string_path, build_dir_path)) {
                Ok(_) => {},
                Err(error) => println!("{}", error)
            }

        },
    }

    return build_dir.join("build.exe")
}

#[derive(Debug, Default, Clone)]
pub enum Mode {
    #[default]
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

#[derive(Debug, Default)]
pub struct Builder {
    mode: Mode,
    constants: HashMap<String, String>,
    body: String,
}
impl Builder {
    pub fn new(mode: Mode) -> Self {
        let mut s = Self::default();
        s.mode = mode;
        return s;
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

    for (name, value) in &builder.constants {
        body.pushln(format!(
            "@{} = private constant [{} x i8] c\"{:?}\"",
            name,
            value.len(),
            value
        ));
    }
}
