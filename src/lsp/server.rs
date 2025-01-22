use std::time::UNIX_EPOCH;

use super::logger::Logger;

mod start;

pub struct LSPServer {
    pub(super) logger: Logger,
}
impl LSPServer {
    pub fn new() -> Self {
        let mut logger = Logger::new("eclipse-analyzer");

        let timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        logger.write(format!("started: {timestamp}"));

        Self { logger }
    }
}
