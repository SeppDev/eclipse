use std::collections::HashMap;

use super::node::IRFunction;

static CHARS: [char; 4] = ['a', 'b', 'c', 'd'];

pub struct IRProgram {
    pub functions: HashMap<String, IRFunction>,
    count: usize
}
impl IRProgram {
    pub fn new() -> Self {
        Self {
            count: 0,
            functions: HashMap::new()
        }
    }
    pub fn generate(&mut self) -> String {
        self.count += 1;
        let count = self.count;

    }
}