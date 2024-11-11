use super::node::IRFunction;

#[derive(Debug)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}
impl IRProgram {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}
