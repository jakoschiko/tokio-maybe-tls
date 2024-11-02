use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
#[cfg(feature = "tokio-native-tls")]
pub(crate) use tokio_native_tls::TlsStream;
#[cfg(feature = "tokio-rustls")]
pub(crate) use tokio_rustls::client::TlsStream;

use crate::MaybeTlsStream;

impl<S> From<TlsStream<S>> for MaybeTlsStream<S> {
    fn from(stream: TlsStream<S>) -> Self {
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
            Self::Plain(ref mut s) => Pin::new(s).poll_read(cx, buf),
            Self::Tls(ref mut s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for MaybeTlsStream<S> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        match self.get_mut() {
            Self::Plain(ref mut s) => Pin::new(s).poll_write(cx, buf),
            Self::Tls(ref mut s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(ref mut s) => Pin::new(s).poll_flush(cx),
            Self::Tls(ref mut s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(ref mut s) => Pin::new(s).poll_shutdown(cx),
            Self::Tls(ref mut s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}
