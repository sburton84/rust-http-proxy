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
};

pub struct State {
    tunnel: bool,
    mitm: bool,
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
            state: Arc::new(Mutex::new(State {
                tunnel: false,
                mitm: false,
            })),
        }
    }

    pub async fn serve(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let http = Http::new();

        http.serve_connection(
            self.socket.clone(),
            ProxyService::new(self.state.clone(), self.connector.clone()),
        ).await?;

        Ok(())
    }
}
