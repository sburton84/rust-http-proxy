use {native_tls::Identity, std::net::SocketAddr};

pub struct Config {
    pub listeners: Vec<Listener>,
}

pub struct Listener {
    pub addr: SocketAddr,
    pub type_: ListenerType,
}

pub enum ListenerType {
    Plain,
    Tls { identity: Identity },
}
