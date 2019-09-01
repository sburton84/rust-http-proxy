use std::net::{SocketAddr, IpAddr};
use rust_http_proxy::Proxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr: SocketAddr = ("127.0.0.1".parse::<IpAddr>()?, 8888).into();

    let proxy = Proxy::bind(&addr).unwrap();
    proxy.serve().await;

    Ok(())
}
