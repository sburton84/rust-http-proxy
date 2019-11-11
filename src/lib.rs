use {
    std::net::SocketAddr,
    tokio::net::TcpListener,
    std::io,
    log::error,
};

mod connection;
mod service;
mod utils;

pub struct Proxy {
    listener: TcpListener,
}

impl Proxy {
    pub async fn bind(addr: &SocketAddr) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;

        Ok(Proxy {
            listener,
        })
    }

    pub async fn serve(&mut self) -> io::Result<()> {
        while let Ok((socket, _)) = self.listener.accept().await {
            tokio::spawn(async {
                match connection::Connection::new(socket).serve().await {
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
