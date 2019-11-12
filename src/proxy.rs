use {
    crate::connection::Connection,
    log::error,
    std::io,
    std::net::SocketAddr,
    tokio::net::TcpListener,
    hyper::client::HttpConnector,
    hyper_tls::HttpsConnector,
};

pub struct Proxy {
    listener: TcpListener,
    connector: HttpsConnector<HttpConnector>,
}

impl Proxy {
    pub async fn bind(addr: &SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        let connector = HttpsConnector::new()?;

        Ok(Proxy {
            listener,
            connector,
        })
    }

    pub async fn serve(&mut self) -> io::Result<()> {
        while let Ok((socket, _)) = self.listener.accept().await {
            let connector = self.connector.clone();

            tokio::spawn(async {
                match Connection::new(socket, connector).serve().await {
                    Ok(()) => {}
                    Err(e) => {
                        error!("Error in connection: {:#?}", e);
                    }
                }
            });
        }

        return Ok(());
    }
}
