use super::node::IRFunction;


#[derive(Debug)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
    pub static_strings: Vec<(String, String)>
}