use std::net::{SocketAddr, IpAddr};
use rust_http_proxy::Proxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Bind to port 8888
    let addr: SocketAddr = ("127.0.0.1".parse::<IpAddr>()?, 8888).into();

    let mut proxy = Proxy::new().unwrap();
    proxy.add_listener(&addr);

    // Begin serving the proxy
    proxy.serve().await?;

    Ok(())
}
