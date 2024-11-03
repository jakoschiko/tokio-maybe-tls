//! # Tokio extensions
//!
//! This module gathers tools to manipulate streams based on tokio
//! extensions [`AsyncRead`] and [`AsyncWrite`].

use std::{
    io::Result,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::{MaybeTlsStream, Stream};

/// Tokio extensions trait.
///
/// This trait is just an alias for [`AsyncRead`], [`AsyncWrite`] and
/// [`Unpin`], so that it can be used inside a [`Box`]. This trait is
/// used by the plain and the TLS stream variants.
pub trait TokioExt: AsyncRead + AsyncWrite + Unpin {}

/// Tokio extensions automatic implementation.
///
/// Everything that is [`AsyncRead`] + [`AsyncWrite`] + [`Unpin`]
/// automatically implements [`TokioExt`].
impl<T: AsyncRead + AsyncWrite + Unpin> TokioExt for T {}

/// Concrete wrapper for tokio extensions.
///
/// This structure is a simple wrapper around tokio extensions
/// [`TokioExt`]. It gather streams that share common tokio
/// extensions.
pub struct Tokio<S: TokioExt>(PhantomData<S>);

/// [`Stream`] implementation for [`Tokio`] structure.
///
/// The plain type is generic, whereas the TLS type is dynamic so that
/// it can be adjusted at runtime.
impl<S: TokioExt> Stream for Tokio<S> {
    type Plain = S;
    type Tls = Box<dyn TokioExt>;
}

/// Specific implementations for the tokio extensions-based
/// [`MaybeTlsStream`].
impl<S: TokioExt> MaybeTlsStream<Tokio<S>> {
    /// Creates a [`MaybeTlsStream::Tls`] variant from the given tokio
    /// extensions-based stream.
    pub fn tokio_tls<T: TokioExt + 'static>(stream: T) -> Self {
        Self::Tls(Box::new(stream))
    }
}

/// [`AsyncRead`] implementation of the tokio extensions-based
/// [`MaybeTlsStream`].
impl<S: Stream> AsyncRead for MaybeTlsStream<S>
where
    S::Plain: TokioExt,
    S::Tls: TokioExt,
{
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

/// [`AsyncWrite`] implementation of the tokio extensions-based
/// [`MaybeTlsStream`].
impl<S: Stream> AsyncWrite for MaybeTlsStream<S>
where
    S::Plain: TokioExt,
    S::Tls: TokioExt,
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

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_shutdown(cx),
            Self::Tls(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}
