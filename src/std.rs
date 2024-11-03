//! # Standard extensions
//!
//! This module gathers tools to manipulate streams based on standard
//! extensions [`Read`] and [`Write`].

use std::{
    io::{Read, Result, Write},
    marker::PhantomData,
};

use crate::{MaybeTlsStream, Stream};

/// Standard extensions trait.
///
/// This trait is just an alias for [`Read`] + [`Write`], so that it
/// can be used inside a [`Box`]. This trait is used by the plain and
/// the TLS stream variants.
pub trait StdExt: Read + Write {}

/// Standard extensions automatic implementation.
///
/// Everything that is [`Read`] + [`Write`] automatically implements
/// [`StdExt`].
impl<T: Read + Write> StdExt for T {}

/// Concrete wrapper for standard extensions.
///
/// This structure is a simple wrapper around standard extensions
/// [`StdExt`]. It gather streams that share common standard
/// extensions.
pub struct Std<S: StdExt>(PhantomData<S>);

/// [`Stream`] implementation for [`Std`] structure.
///
/// The plain type is generic, whereas the TLS type is dynamic so that
/// it can be adjusted at runtime.
impl<S: StdExt> Stream for Std<S> {
    type Plain = S;
    type Tls = Box<dyn StdExt>;
}

/// Specific implementations for the standard extensions-based
/// [`MaybeTlsStream`].
impl<S: StdExt> MaybeTlsStream<Std<S>> {
    /// Creates a [`MaybeTlsStream::Tls`] variant from the given
    /// standard extensions-based stream.
    pub fn std_tls<T: StdExt + 'static>(stream: T) -> Self {
        Self::Tls(Box::new(stream))
    }
}

/// [`Read`] implementation of the standard extensions-based
/// [`MaybeTlsStream`].
impl<S: Stream> Read for MaybeTlsStream<S>
where
    S::Plain: StdExt,
    S::Tls: StdExt,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Self::Plain(stream) => stream.read(buf),
            Self::Tls(stream) => stream.read(buf),
        }
    }
}

/// [`Write`] implementation of the standard extensions-based
/// [`MaybeTlsStream`].
impl<S: Stream> Write for MaybeTlsStream<S>
where
    S::Plain: StdExt,
    S::Tls: StdExt,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            Self::Plain(stream) => stream.write(buf),
            Self::Tls(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            Self::Plain(stream) => stream.flush(),
            Self::Tls(stream) => stream.flush(),
        }
    }
}
