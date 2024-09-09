pub struct Writer {
    pub body: String,
}
impl Writer {
    pub fn new() -> Self {
        Self {
            body: String::new(),
        }
    }
    pub fn push(&mut self, body: &String) {
        self.body.push_str(&body);
    }
    pub fn push_str(&mut self, body: &str) {
        self.body.push_str(body);
    }
    pub fn label(&mut self, label: &String) {
        self.body.push_str(&format!("{}:\n", label))
    }
    pub fn add_operation(&mut self, operation: String) {
        self.body.push_str(&format!("\t{}\n", operation))
    }
    pub fn add_operation_str(&mut self, operation: &str) {
        self.body.push_str(&format!("\t{}\n", operation))
    }
    pub fn writer(&mut self, writer: Self) {
        self.body.push_str(&writer.body);
    }
}
