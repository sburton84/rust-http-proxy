use {
    crate::{
        service::ProxyService,
        utils,
    },
    hyper::{
        client::HttpConnector,
        server::conn::Http,
    },
    hyper_tls::HttpsConnector,
    tokio::net::TcpStream,
    std::sync::{
        Arc, Mutex,
    },
    log::debug,
    tokio::io::split,
    tokio::io::AsyncReadExt,
};

#[derive(Clone)]
pub enum State{
    Proxy,
    Tunnel(String),
    Mitm(String),
}

pub struct Connection {
    socket: utils::SyncSocket,
    connector: HttpsConnector<HttpConnector>,
    state: Arc<Mutex<State>>,
}

impl Connection {
    pub fn new(socket: TcpStream, connector: HttpsConnector<HttpConnector>) -> Self {
        Connection {
            socket: utils::SyncSocket::new(socket),
            connector: connector,
            state: Arc::new(Mutex::new(State::Proxy)),
        }
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let http = Http::new();

        http.serve_connection(
            self.socket.clone(),
            ProxyService::new(self.state.clone(), self.connector.clone()),
        ).await?;

        let state = self.state.lock().unwrap().clone();
        match state {
            State::Proxy => {
                debug!("Request proxied, nothing else to do");
            },
            State::Tunnel(uri) => {
                self.tunnel(&uri).await?;
            },
            State::Mitm(_uri) => {},
        };

        Ok(())
    }

    async fn tunnel(&self, uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(uri).await?;
        let (mut client_read, mut client_write) = split(self.socket.clone());
        let (mut server_read, mut server_write) = split(stream);

        tokio::spawn(async move {
            client_read.copy(&mut server_write).await;
        });
        tokio::spawn(async move {
            server_read.copy(&mut client_write).await;
        });

        Ok(())
    }
}
