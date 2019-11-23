use {
    native_tls::Identity,
    rust_http_proxy::Proxy,
    std::net::{IpAddr, SocketAddr},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Bind to port 443
    let addr: SocketAddr = ("127.0.0.1".parse::<IpAddr>()?, 443).into();

    let mut proxy = Proxy::new().unwrap();

    let der = include_bytes!("tls_identity.p12");
    let cert = Identity::from_pkcs12(der, "password")?;

    proxy.add_tls_listener(&addr, cert);

    // Begin serving the proxy
    proxy.serve().await?;

    Ok(())
}
