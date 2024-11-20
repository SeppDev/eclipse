use super::node::IRFunction;


#[derive(Debug)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}