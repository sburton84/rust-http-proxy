use {
    crate::{
        config::{Config, Listener, ListenerType},
        connection::Connection,
        utils::TcpOrTlsStream,
    },
    futures::{TryFutureExt, TryStreamExt, stream::select_all, Stream, StreamExt},
    hyper::client::HttpConnector,
    hyper_tls::HttpsConnector,
    log::error,
    native_tls::{Identity, TlsAcceptor as NativeTlsAcceptor},
    std::{net::SocketAddr, pin::Pin},
    tokio::net::TcpListener,
    tokio_tls::{
        TlsAcceptor,
    },
};

pub struct Proxy {
    config: Config,
    connector: HttpsConnector<HttpConnector>,
}

impl Proxy {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Proxy {
            config: Config {
                listeners: Vec::new(),
            },
            connector: HttpsConnector::new(),
        })
    }

    pub fn add_listener(&mut self, addr: &SocketAddr) {
        self.config.listeners.push(Listener {
            addr: *addr,
            type_: ListenerType::Plain,
        });
    }

    pub fn add_tls_listener(&mut self, addr: &SocketAddr, identity: Identity) {
        self.config.listeners.push(Listener {
            addr: *addr,
            type_: ListenerType::Tls { identity: identity },
        });
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut listeners: Vec<Pin<Box<dyn Stream<Item=Result<TcpOrTlsStream, Box<dyn std::error::Error>>>>>> = Vec::new();

        for listener in self.config.listeners {
            // Push each stream of incoming connections onto the vector
            match listener.type_ {
                ListenerType::Plain => {
                    listeners.push(
                        TcpListener::bind(listener.addr).await?.map_err(|e| {
                            Box::new(e) as Box<dyn std::error::Error>
                        }).map_ok(|sock| {
                            TcpOrTlsStream::Tcp(sock)
                        }).boxed()
                    );
                }
                ListenerType::Tls { identity } => {
                    let tls_cx = NativeTlsAcceptor::builder(identity).build()?;
                    let acceptor = TlsAcceptor::from(tls_cx);

                    let stream = TcpListener::bind(listener.addr).await?.map_err(|e| {
                        Box::new(e) as Box<dyn std::error::Error>
                    });

                    let stream = stream.and_then(move |sock| {
                        let acceptor = acceptor.clone();

                        async move {
                            let acceptor = acceptor.clone();

                            acceptor.accept(sock).map_err(|e| {
                                Box::new(e) as Box<dyn std::error::Error>
                            }).await
                        }
                    });

                    let stream = stream.map_ok(|sock| {
                        TcpOrTlsStream::Tls(sock)
                    }).boxed();

                    listeners.push(stream);
                }
            }
        }

        // Select between all the listener streams, producing a single stream of incoming connections
        let mut select = select_all(listeners);

        // Iterate over incoming connections
        while let Some(Ok(socket)) = select.next().await {
            let connector = self.connector.clone();

            // Spawn a new handler for this connection
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
