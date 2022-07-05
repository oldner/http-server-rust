use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

/*
GET /user?id=1 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
