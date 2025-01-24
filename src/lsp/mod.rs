use server::LSPServer;

mod errors;
mod json;
mod logger;
mod message;
mod server;

pub fn init() -> ! {
    let server = LSPServer::new();

    server.start()
}
