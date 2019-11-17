use std::net::SocketAddr;

pub struct Config {
    pub listeners: Vec<Listener>
}

pub struct Listener {
    pub addr: SocketAddr,
    pub tls: bool,
}
