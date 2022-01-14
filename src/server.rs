use std::io::Read;
use std::convert::TryFrom;
use crate::http::Request;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr 
        }
    }
    
    pub fn run(self) {
        print!("Server is listening on {}\n", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(err) => println!("Failed to parse the request: {}", err)
                            }
                        }
                        Err(err) => {
                            println!("Error: {}", err)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }

            }
            
            let res = listener.accept();

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}