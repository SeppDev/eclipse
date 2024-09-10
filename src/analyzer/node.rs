pub enum IRNode {
    
}

pub struct IRFunction {
    path: String,
}

pub struct IRProgram {
    functions: Vec<IRFunction>
}

impl IRProgram {
    pub fn new() -> Self {
        Self {
            functions: Vec::new()
        }
    }
}