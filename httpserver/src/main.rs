use server::Server;

mod server;
mod router;
mod handler;

fn main() { // Start a server
    let server = Server::new("localhost:3000");
    //Run the server
    server.run();
}