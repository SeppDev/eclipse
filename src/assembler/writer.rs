pub struct Writer {
    pub body: String,
}
impl Writer {
    pub fn new() -> Self {
        let mut writer = Self {
            body: String::new(),
        };
        
        writer.push_str("bits 64\n");
        writer.push_str("global main\n");
        writer.push_str("extern printf, puts, exit\n\n");    
        writer.push_str("section .text\n"); 
        
        writer
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
