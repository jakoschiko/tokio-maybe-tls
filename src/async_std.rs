//! Convenience wrapper for streams that allows to choose between plain and TLS streams
//! at runtime.
//!
//! A stream can be any type that implements [`AsyncRead`], [`AsyncWrite`] and [`Unpin`].

use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(feature = "async-std-native-tls")]
pub(crate) use async_native_tls::TlsStream;
#[cfg(feature = "async-std-rustls")]
pub(crate) use async_tls::client::TlsStream;
use futures_io::{AsyncRead, AsyncWrite};

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
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
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

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(ref mut s) => Pin::new(s).poll_close(cx),
            Self::Tls(ref mut s) => Pin::new(s).poll_close(cx),
        }
    }
}
