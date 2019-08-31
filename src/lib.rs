use std::net::{SocketAddr, TcpListener};
use std::io;
use log::error;

pub struct Proxy {
    listener: TcpListener,
}

impl Proxy {
    fn bind(addr: &SocketAddr) -> io::Result<Self> {
        let listener = match TcpListener::bind(addr) {
            Ok(l) => l,
            Err(e) => {
                error!("Error binding to socket: {}", e);
                return Err(e);
            }
        };

        Ok(Proxy {
            listener: listener,
        })
    }
}
