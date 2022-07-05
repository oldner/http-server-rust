use crate::http::Request;
use std::convert::TryFrom;
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap(); // unwrap returns result value if Ok, otherwise determines the programs and logs the error

        println!("Listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024]; // why we created a buf here is because stream.read() has to be implemented (because of TCP Listener). We put 1024 because it is very enough for this simple server.
                    match stream.read(&mut buf) {
                        Ok(value) => {
                            println!("{}", value);
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));

                            match Request::try_from(&buf[..]) {
                                Ok(req) => {}
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }

                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
