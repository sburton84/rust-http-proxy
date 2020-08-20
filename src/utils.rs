use {
    std::{
        io,
        pin::Pin,
        task::Poll,
        sync::{Arc, Mutex},
    },
    tokio::{
        io::{AsyncRead, AsyncWrite},
        net::TcpStream,
    },
    tokio_tls::TlsStream,
};

pub enum TcpOrTlsStream {
    Tcp(TcpStream),
    Tls(TlsStream<TcpStream>),
}

impl AsyncRead for TcpOrTlsStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match &mut  *self {
            TcpOrTlsStream::Tcp(stream) => {
                AsyncRead::poll_read(Pin::new(stream), cx, buf)
            },
            TcpOrTlsStream::Tls(stream) => {
                AsyncRead::poll_read(Pin::new(stream), cx, buf)
            },
        }
    }
}

impl AsyncWrite for TcpOrTlsStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        match &mut  *self {
            TcpOrTlsStream::Tcp(stream) => {
                AsyncWrite::poll_write(Pin::new(stream), cx, buf)
            },
            TcpOrTlsStream::Tls(stream) => {
                AsyncWrite::poll_write(Pin::new(stream), cx, buf)
            },
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        match &mut *self {
            TcpOrTlsStream::Tcp(stream) => {
                AsyncWrite::poll_flush(Pin::new(stream), cx)
            },
            TcpOrTlsStream::Tls(stream) => {
                AsyncWrite::poll_flush(Pin::new(stream), cx)
            },
        }
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        match &mut *self {
            TcpOrTlsStream::Tcp(stream) => {
                AsyncWrite::poll_shutdown(Pin::new(stream), cx)
            },
            TcpOrTlsStream::Tls(stream) => {
                AsyncWrite::poll_shutdown(Pin::new(stream), cx)
            },
        }
    }
}

/// A wrapper around a TcpStream that can be used to allow the TcpStream to be
/// passed between threads
#[derive(Clone)]
pub struct SyncSocket {
    stream: Arc<Mutex<TcpOrTlsStream>>,
}

impl SyncSocket {
    pub fn new(stream: TcpOrTlsStream) -> Self {
        SyncSocket {
            stream: Arc::new(Mutex::new(stream)),
        }
    }
}

impl AsyncRead for SyncSocket {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        AsyncRead::poll_read(Pin::new(&mut *self.stream.lock().unwrap()), cx, buf)
    }
}

impl AsyncWrite for SyncSocket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        AsyncWrite::poll_write(Pin::new(&mut *self.stream.lock().unwrap()), cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        AsyncWrite::poll_flush(Pin::new(&mut *self.stream.lock().unwrap()), cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut *self.stream.lock().unwrap()), cx)
    }
}
