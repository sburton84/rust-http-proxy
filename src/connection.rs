use {
    crate::{
        service::ProxyService,
        utils,
    },
    hyper::server::conn::Http,
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
    state: Arc<Mutex<State>>,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            socket: utils::SyncSocket::new(socket),
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
            ProxyService::new(self.state.clone()),
        ).await?;

        Ok(())
    }
}
