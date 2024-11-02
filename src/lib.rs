//! Convenience wrapper for streams that allow to switch between plain TCP and different TLS
//! implementations at runtime.
//!
//! A stream can be anything that implement [`AsyncRead`], [`AsyncWrite`] and [`Unpin`].

#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{
    convert::Infallible,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A stream that might be encrypted with TLS.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum MaybeTlsStream<S> {
    /// Unencrypted stream.
    Plain(S),
    /// Encrypted stream.
    Tls(TlsStream<S>),
}

#[cfg(feature = "native-tls")]
#[cfg_attr(docsrs, doc(cfg(feature = "native-tls")))]
impl<S> From<tokio_native_tls::TlsStream<S>> for MaybeTlsStream<S> {
    fn from(value: tokio_native_tls::TlsStream<S>) -> Self {
        Self::Tls(value.into())
    }
}

#[cfg(feature = "rustls")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
impl<S> From<tokio_rustls::client::TlsStream<S>> for MaybeTlsStream<S> {
    fn from(value: tokio_rustls::client::TlsStream<S>) -> Self {
        Self::Tls(value.into())
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for MaybeTlsStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Plain(ref mut s) => Pin::new(s).poll_read(cx, buf),
            Self::Tls(ref mut s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for MaybeTlsStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match self.get_mut() {
            Self::Plain(ref mut s) => Pin::new(s).poll_write(cx, buf),
            Self::Tls(ref mut s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::Plain(ref mut s) => Pin::new(s).poll_flush(cx),
            Self::Tls(ref mut s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::Plain(ref mut s) => Pin::new(s).poll_shutdown(cx),
            Self::Tls(ref mut s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}

/// A stream that is encrypted with TLS.
///
/// This enum is non-exhaustive because additional feature-gated implementations might be added
/// in the future.
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum TlsStream<S> {
    /// Hidden variant that exist for technical reasons.
    ///
    /// Users of this crate must not construct or match this variant.
    #[doc(hidden)]
    None(PhantomData<S>, Infallible),
    /// Encrypted stream using `native-tls`.
    #[cfg(feature = "native-tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "native-tls")))]
    NativeTls(tokio_native_tls::TlsStream<S>),
    /// Encrypted stream using `rustls`.
    #[cfg(feature = "rustls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
    Rustls(tokio_rustls::client::TlsStream<S>),
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for TlsStream<S> {
    #[allow(unused_variables)]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::None(_, _) => Poll::Pending,
            #[cfg(feature = "native-tls")]
            Self::NativeTls(s) => Pin::new(s).poll_read(cx, buf),
            #[cfg(feature = "rustls")]
            Self::Rustls(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for TlsStream<S> {
    #[allow(unused_variables)]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match self.get_mut() {
            Self::None(_, _) => Poll::Pending,
            #[cfg(feature = "native-tls")]
            Self::NativeTls(s) => Pin::new(s).poll_write(cx, buf),
            #[cfg(feature = "rustls")]
            Self::Rustls(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    #[allow(unused_variables)]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::None(_, _) => Poll::Pending,
            #[cfg(feature = "native-tls")]
            Self::NativeTls(s) => Pin::new(s).poll_flush(cx),
            #[cfg(feature = "rustls")]
            Self::Rustls(s) => Pin::new(s).poll_flush(cx),
        }
    }

    #[allow(unused_variables)]
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::None(_, _) => Poll::Pending,
            #[cfg(feature = "native-tls")]
            Self::NativeTls(s) => Pin::new(s).poll_shutdown(cx),
            #[cfg(feature = "rustls")]
            Self::Rustls(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}

#[cfg(feature = "native-tls")]
#[cfg_attr(docsrs, doc(cfg(feature = "native-tls")))]
impl<S> From<tokio_native_tls::TlsStream<S>> for TlsStream<S> {
    fn from(value: tokio_native_tls::TlsStream<S>) -> Self {
        Self::NativeTls(value)
    }
}

#[cfg(feature = "rustls")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
impl<S> From<tokio_rustls::client::TlsStream<S>> for TlsStream<S> {
    fn from(value: tokio_rustls::client::TlsStream<S>) -> Self {
        Self::Rustls(value)
    }
}
