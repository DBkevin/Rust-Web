mod server;
mod router;
mod handle;
use server::Server;
fn main() {
    let server=Server::new("localhost:3000");
    server.run();

}
