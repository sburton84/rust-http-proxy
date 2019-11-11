use std::task::Poll;
use std::{
    io,
    sync::{Arc, Mutex},
    pin::Pin,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};

/// A wrapper around a TcpStream that can be used to allow the TcpStream to be
/// passed between threads
#[derive(Clone)]
pub struct SyncSocket {
    stream: Arc<Mutex<TcpStream>>,
}

impl SyncSocket {
    pub fn new(stream: TcpStream) -> SyncSocket {
        SyncSocket { stream: Arc::new(Mutex::new(stream)) }
    }
}

impl AsyncRead for SyncSocket {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        AsyncRead::poll_read(Pin::new(&mut *self.stream.lock().unwrap()), cx, buf)
    }
}

impl AsyncWrite for SyncSocket {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &[u8]) -> Poll<Result<usize, io::Error>> {
        AsyncWrite::poll_write(Pin::new(&mut *self.stream.lock().unwrap()), cx,buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), io::Error>> {
        AsyncWrite::poll_flush(Pin::new(&mut *self.stream.lock().unwrap()), cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), io::Error>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut *self.stream.lock().unwrap()), cx)
    }
}
