//! # Async std extensions
//!
//! This module exposes an async version of [`MaybeTlsStream`], based
//! on `async-std` runtime. Encryption can be done with
//! [`futures_rustls`] or [`async_native_tls`], and I/O is based on
//! [`futures`] extensions [`AsyncRead`] and [`AsyncWrite`].

use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

use futures_io::{AsyncRead, AsyncWrite};

/// A stream that might be encrypted with TLS.
#[derive(Debug)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum MaybeTlsStream<S> {
    /// Unencrypted stream.
    Plain(S),
    /// Stream encrypted with [`tokio_rustls`].
    #[cfg(feature = "futures-rustls")]
    FuturesRustls(futures_rustls::client::TlsStream<S>),
    /// Stream encrypted with [`async_native_tls`].
    #[cfg(feature = "async-native-tls")]
    AsyncNativeTls(async_native_tls::TlsStream<S>),
}

impl<S> MaybeTlsStream<S> {
    /// Creates an unencrypted stream.
    pub fn plain(stream: impl Into<S>) -> Self {
        Self::Plain(stream.into())
    }

    /// Create an encrypted stream.
    pub fn tls(stream: impl Into<Self>) -> Self {
        stream.into()
    }
}

#[cfg(feature = "futures-rustls")]
impl<S> From<futures_rustls::client::TlsStream<S>> for MaybeTlsStream<S> {
    fn from(stream: futures_rustls::client::TlsStream<S>) -> Self {
        Self::FuturesRustls(stream)
    }
}

#[cfg(feature = "async-native-tls")]
impl<S> From<async_native_tls::TlsStream<S>> for MaybeTlsStream<S> {
    fn from(stream: async_native_tls::TlsStream<S>) -> Self {
        Self::AsyncNativeTls(stream)
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for MaybeTlsStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            #[cfg(feature = "futures-rustls")]
            Self::FuturesRustls(stream) => Pin::new(stream).poll_read(cx, buf),
            #[cfg(feature = "async-native-tls")]
            Self::AsyncNativeTls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for MaybeTlsStream<S> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            #[cfg(feature = "futures-rustls")]
            Self::FuturesRustls(stream) => Pin::new(stream).poll_write(cx, buf),
            #[cfg(feature = "async-native-tls")]
            Self::AsyncNativeTls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_flush(cx),
            #[cfg(feature = "futures-rustls")]
            Self::FuturesRustls(stream) => Pin::new(stream).poll_flush(cx),
            #[cfg(feature = "async-native-tls")]
            Self::AsyncNativeTls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_close(cx),
            #[cfg(feature = "futures-rustls")]
            Self::FuturesRustls(stream) => Pin::new(stream).poll_close(cx),
            #[cfg(feature = "async-native-tls")]
            Self::AsyncNativeTls(stream) => Pin::new(stream).poll_close(cx),
        }
    }
}
