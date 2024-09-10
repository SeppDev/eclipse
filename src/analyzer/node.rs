pub enum IRNode {
    
}

pub struct IRFunction {
    pub path: String,
}

pub struct IRProgram {
    pub functions: Vec<IRFunction>
}

impl IRProgram {
    pub fn new() -> Self {
        Self {
            functions: Vec::new()
        }
    }
    pub fn push_functions(&mut self, functions: Vec<IRFunction>) {
        for func in functions {
            self.functions.push(func);
        }
    }
}