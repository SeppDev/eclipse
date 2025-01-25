use std::process::exit;

use std::io::{self, BufRead, Read};

use crate::lsp::errors::LSPResult;
use crate::lsp::json;
use crate::lsp::message::{ClientMessage, Message};
use crate::lsp::types::RequestMessage;

use super::LSPServer;

impl LSPServer {
    fn run(&mut self) -> LSPResult<()> {
        let input = io::stdin();
        // let ouput = io::stdout();

        loop {
            let mut input_lock = input.lock();

            let mut length_buffer = String::new();
            input_lock.read_line(&mut length_buffer).unwrap();
            let content_length = length_buffer
                .split("Content-Length: ")
                .last()
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap();

            input_lock.read_line(&mut String::new())?;
            let length: usize = content_length.parse()?;

            let mut buffer = Vec::new();
            buffer.resize(length, 0);

            input_lock.read_exact(&mut buffer)?;

            let string = String::from_utf8(buffer)?;
            let object = json::from_str(string);

            let json = object.clone().stringify();
            self.logger.write(json);

            let message = ClientMessage::from_json(object)?;

            self.logger.write(format!("id: {}", message.id));
        }
    }
    pub fn start(mut self) -> ! {
        let message = match self.run() {
            Ok(_) => "LSP stopped".to_string(),
            Err(error) => format!("LSP crashed: {error}"),
        };
        self.logger.write(message);
        exit(0)
    }
}
