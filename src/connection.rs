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
    log::{debug, warn},
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

        // Handle the initial HTTP request. If this is a plaintext request then this will
        // also make the upstream request and forward the response. If it's a CONNECT request
        // it will set the appropriate state so we can continue to handle it here.
        http.serve_connection(
            self.socket.clone(),
            ProxyService::new(self.state.clone(), self.connector.clone()),
        ).await?;

        let state = self.state.lock().unwrap().clone();
        match state {
            State::Proxy => {
                // Plaintext request was already handled
                debug!("Request proxied, nothing else to do");
            },
            State::Tunnel(uri) => {
                // CONNECT request which should open a tunnel to the upstream server
                self.tunnel(&uri).await?;
            },
            State::Mitm(uri) => {
                // CONNECT request where we should also MitM the TLS tunnel
                self.mitm(&uri).await?
            },
        };

        Ok(())
    }

    async fn tunnel(&self, uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Connect to the upstream server
        let stream = TcpStream::connect(uri).await?;

        let (mut client_read, mut client_write) = split(self.socket.clone());
        let (mut server_read, mut server_write) = split(stream);

        // Spawn futures to copy all subsequent data from the client to the server
        // and from the server to the client
        tokio::spawn(async move {
            if let Err(e) = client_read.copy(&mut server_write).await {
                warn!("Error copy data from client to server: {}", e);
            }
        });
        tokio::spawn(async move {
            if let Err(e) = server_read.copy(&mut client_write).await {
                warn!("Error copy data from server to client: {}", e);
            }
        });

        Ok(())
    }

    async fn mitm(&self, uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Connect to the upstream server
        let stream = TcpStream::connect(uri).await?;


        Ok(())
    }
}
