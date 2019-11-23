use rust_http_proxy::Proxy;
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Bind to port 80
    let addr: SocketAddr = ("127.0.0.1".parse::<IpAddr>()?, 80).into();

    let mut proxy = Proxy::new().unwrap();
    proxy.add_listener(&addr);

    // Begin serving the proxy
    proxy.serve().await?;

    Ok(())
}
