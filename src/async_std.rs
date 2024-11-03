use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

use async_std::{
    io::{Read, Write},
    net::TcpStream,
};

pub trait TlsStream<S: Read + Write + Unpin>: Read + Write + Unpin {}

impl<T: Read + Write + Unpin, S: Read + Write + Unpin> TlsStream<S> for T {}

/// A stream that might be encrypted with TLS.
#[allow(clippy::large_enum_variant)]
pub enum MaybeTlsStream<S> {
    /// Unencrypted stream.
    Plain(S),
    /// Encrypted stream.
    Tls(Box<dyn TlsStream<S>>),
}

impl<S: Read + Write + Unpin> MaybeTlsStream<S> {
    pub fn plain(stream: S) -> Self {
        Self::Plain(stream)
    }

    pub fn tls(stream: Box<dyn TlsStream<S>>) -> Self {
        Self::Tls(stream)
    }
}

impl<S: Read + Write + Unpin> Read for MaybeTlsStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            Self::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl<S: Read + Write + Unpin> Write for MaybeTlsStream<S> {
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

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_close(cx),
            Self::Tls(stream) => Pin::new(stream).poll_close(cx),
        }
    }
}

impl From<TcpStream> for MaybeTlsStream<TcpStream> {
    fn from(stream: TcpStream) -> Self {
        MaybeTlsStream::plain(stream)
    }
}

#[cfg(feature = "async-std-rustls")]
impl From<futures_rustls::client::TlsStream<TcpStream>> for MaybeTlsStream<TcpStream> {
    fn from(stream: futures_rustls::client::TlsStream<TcpStream>) -> Self {
        MaybeTlsStream::tls(Box::new(stream))
    }
}

#[cfg(feature = "async-std-native-tls")]
impl From<async_native_tls::TlsStream<TcpStream>> for MaybeTlsStream<TcpStream> {
    fn from(stream: async_native_tls::TlsStream<TcpStream>) -> Self {
        MaybeTlsStream::tls(Box::new(stream))
    }
}
