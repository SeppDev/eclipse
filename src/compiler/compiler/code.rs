pub struct Code {
    pub body: String,
}
impl Code {
    pub fn new() -> Self {
        Self {
            body: String::new(),
        }
    }
    pub fn push(&mut self, string: String) {
        self.body.push_str(string.as_str())
    }
    pub fn push_str(&mut self, string: &str) {
        self.push(String::from(string));
    }

    pub fn add_operation_str(&mut self, operation: &str) {
        self.push(format!("\t{}\n", operation));
    }
    pub fn add_operation(&mut self, operation: String) {
        self.push(format!("\t{}\n", operation));
    }
    pub fn next_line(&mut self) {
        self.body.push_str("\n");
    }
    pub fn label(&mut self, key: String) {
        self.push(format!("{}:\n", key));
    }

    pub fn function(&mut self, name: &String, content: String, stack_size: usize) {
        self.push(format!("{}:\n", name));
        self.add_operation_str("push rbp");
        self.add_operation_str("mov rbp, rsp");
        self.add_operation(format!("sub rsp, {}", stack_size + 16));
        self.push(content);
        self.add_operation(format!("add rsp, {}", stack_size + 16));
        self.add_operation_str("mov rsp, rbp");
        self.add_operation_str("pop rbp");
        self.add_operation_str("ret\n");
    }
}