use server::LSPServer;

pub mod logger;
mod server;

pub fn init() -> ! {
    let server = LSPServer::new();

    server.run()
}
