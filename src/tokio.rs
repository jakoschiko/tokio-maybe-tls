use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::TcpStream,
};

pub trait TlsStream<S: AsyncRead + AsyncWrite + Unpin>: AsyncRead + AsyncWrite + Unpin {}

impl<T: AsyncRead + AsyncWrite + Unpin, S: AsyncRead + AsyncWrite + Unpin> TlsStream<S> for T {}

/// A stream that might be encrypted with TLS.
#[allow(clippy::large_enum_variant)]
pub enum MaybeTlsStream<S> {
    /// Unencrypted stream.
    Plain(S),
    /// Encrypted stream.
    Tls(Box<dyn TlsStream<S>>),
}

impl<S: AsyncRead + AsyncWrite + Unpin> MaybeTlsStream<S> {
    pub fn plain(stream: S) -> Self {
        Self::Plain(stream)
    }

    pub fn tls(stream: Box<dyn TlsStream<S>>) -> Self {
        Self::Tls(stream)
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for MaybeTlsStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            Self::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for MaybeTlsStream<S> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            Self::Tls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_flush(cx),
            Self::Tls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_shutdown(cx),
            Self::Tls(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

impl From<TcpStream> for MaybeTlsStream<TcpStream> {
    fn from(stream: TcpStream) -> Self {
        MaybeTlsStream::plain(stream)
    }
}

#[cfg(feature = "tokio-rustls")]
impl From<tokio_rustls::client::TlsStream<TcpStream>> for MaybeTlsStream<TcpStream> {
    fn from(stream: tokio_rustls::client::TlsStream<TcpStream>) -> Self {
        MaybeTlsStream::tls(Box::new(stream))
    }
}

#[cfg(feature = "tokio-native-tls")]
impl From<tokio_native_tls::TlsStream<TcpStream>> for MaybeTlsStream<TcpStream> {
    fn from(stream: tokio_native_tls::TlsStream<TcpStream>) -> Self {
        MaybeTlsStream::tls(Box::new(stream))
    }
}
