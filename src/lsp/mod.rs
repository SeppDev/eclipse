use server::LSPServer;

mod errors;
mod json;
mod logger;
mod message;
mod server;
mod types;

pub fn init() -> ! {
    let server = LSPServer::new();

    server.start()
}
