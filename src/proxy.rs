use {
    crate::{
        config::{
            Config,
            Listener,
        },
        connection::Connection,
    },
    log::error,
    std::io,
    std::net::SocketAddr,
    futures::{
        stream::select_all,
        StreamExt,
    },
    tokio::net::TcpListener,
    hyper::client::HttpConnector,
    hyper_tls::HttpsConnector,
};

pub struct Proxy {
    config: Config,
    connector: HttpsConnector<HttpConnector>,
}

impl Proxy {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Proxy {
            config: Config{
                listeners: Vec::new(),
            },
            connector: HttpsConnector::new()?,
        })
    }

    pub fn add_listener(&mut self, addr: &SocketAddr) {
        self.config.listeners.push(Listener{
            addr: *addr,
            tls: false,
        })
    }

    pub async fn serve(&mut self) -> io::Result<()> {
        let mut listeners = Vec::new();

        for listener in &self.config.listeners {
            // Push each stream of incoming connections onto the vector
            listeners.push(TcpListener::bind(listener.addr).await?.incoming());
        }

        let mut select = select_all(listeners);

        while let Some(Ok(socket)) = select.next().await {
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

        Ok(())
    }
}
