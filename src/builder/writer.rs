pub struct Writer {
    pub body: String
}
impl Writer {
    pub fn new() -> Self {
        Self {
            body: String::new()
        }
    }
    pub fn label(&mut self, label: &String) {
        self.body.push_str(&format!("{}:\n", label))
    }
    pub fn add_operation(&mut self, operation: &String) {
        self.body.push_str(&format!("\t{}\n", operation))
    }
}