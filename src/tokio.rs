//! # Tokio extensions
//!
//! This module exposes an async version of [`MaybeTlsStream`], based
//! on `tokio` runtime. Encryption can be done with [`tokio_rustls`]
//! or [`tokio_native_tls`], and I/O is based on [`tokio`] extensions
//! [`AsyncRead`] and [`AsyncWrite`].

use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A stream that might be encrypted with TLS.
#[derive(Debug)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum MaybeTlsStream<S> {
    /// Unencrypted stream.
    Plain(S),
    /// Stream encrypted with [`tokio_rustls`].
    #[cfg(feature = "tokio-rustls")]
    TokioRustls(tokio_rustls::client::TlsStream<S>),
    /// Stream encrypted with [`tokio_native_tls`].
    #[cfg(feature = "tokio-native-tls")]
    TokioNativeTls(tokio_native_tls::TlsStream<S>),
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

#[cfg(feature = "tokio-rustls")]
impl<S> From<tokio_rustls::client::TlsStream<S>> for MaybeTlsStream<S> {
    fn from(stream: tokio_rustls::client::TlsStream<S>) -> Self {
        Self::TokioRustls(stream)
    }
}

#[cfg(feature = "tokio-native-tls")]
impl<S> From<tokio_native_tls::TlsStream<S>> for MaybeTlsStream<S> {
    fn from(stream: tokio_native_tls::TlsStream<S>) -> Self {
        Self::TokioNativeTls(stream)
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
            #[cfg(feature = "tokio-rustls")]
            Self::TokioRustls(stream) => Pin::new(stream).poll_read(cx, buf),
            #[cfg(feature = "tokio-native-tls")]
            Self::TokioNativeTls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for MaybeTlsStream<S> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            #[cfg(feature = "tokio-rustls")]
            Self::TokioRustls(stream) => Pin::new(stream).poll_write(cx, buf),
            #[cfg(feature = "tokio-native-tls")]
            Self::TokioNativeTls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_flush(cx),
            #[cfg(feature = "tokio-rustls")]
            Self::TokioRustls(stream) => Pin::new(stream).poll_flush(cx),
            #[cfg(feature = "tokio-native-tls")]
            Self::TokioNativeTls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_shutdown(cx),
            #[cfg(feature = "tokio-rustls")]
            Self::TokioRustls(stream) => Pin::new(stream).poll_shutdown(cx),
            #[cfg(feature = "tokio-native-tls")]
            Self::TokioNativeTls(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}
