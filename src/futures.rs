//! # Futures extensions
//!
//! This module gathers tools to manipulate streams based on futures
//! extensions [`AsyncRead`] and [`AsyncWrite`].

use std::{
    io::Result,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures_io::{AsyncRead, AsyncWrite};

use crate::{MaybeTlsStream, Stream};

/// Futures extensions trait.
///
/// This trait is just an alias for [`AsyncRead`], [`AsyncWrite`] and
/// [`Unpin`], so that it can be used inside a [`Box`]. This trait is
/// used by the plain and the TLS stream variants.
pub trait FuturesExt: AsyncRead + AsyncWrite + Unpin {}

/// Futures extensions automatic implementation.
///
/// Everything that is [`AsyncRead`] + [`AsyncWrite`] + [`Unpin`]
/// automatically implements [`FuturesExt`].
impl<T: AsyncRead + AsyncWrite + Unpin> FuturesExt for T {}

/// Concrete wrapper for futures extensions.
///
/// This structure is a simple wrapper around futures extensions
/// [`FuturesExt`]. It gather streams that share common futures
/// extensions.
pub struct Futures<S: FuturesExt>(PhantomData<S>);

/// [`Stream`] implementation for [`Futures`] structure.
///
/// The plain type is generic, whereas the TLS type is dynamic so that
/// it can be adjusted at runtime.
impl<S: FuturesExt> Stream for Futures<S> {
    type Plain = S;
    type Tls = Box<dyn FuturesExt>;
}

/// Specific implementations for the futures extensions-based
/// [`MaybeTlsStream`].
impl<S: FuturesExt> MaybeTlsStream<Futures<S>> {
    /// Creates a [`MaybeTlsStream::Tls`] variant from the given
    /// futures extensions-based stream.
    pub fn futures_tls<T: FuturesExt + 'static>(stream: T) -> Self {
        Self::Tls(Box::new(stream))
    }
}

/// [`AsyncRead`] implementation of the futures extensions-based
/// [`MaybeTlsStream`].
impl<S: Stream> AsyncRead for MaybeTlsStream<S>
where
    S::Plain: FuturesExt,
    S::Tls: FuturesExt,
{
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

/// [`AsyncWrite`] implementation of the futures extensions-based
/// [`MaybeTlsStream`].
impl<S: Stream> AsyncWrite for MaybeTlsStream<S>
where
    S::Plain: FuturesExt,
    S::Tls: FuturesExt,
{
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
