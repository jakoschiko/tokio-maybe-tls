//! # Standard extension
//!
//! This module exposes a blocking version of
//! [`MaybeTlsStream`]. Encryption can be done with [`rustls`] or
//! [`native_tls`], and I/O is based on standard extensions [`Read`]
//! and [`Write`].

use std::io::{Read, Result, Write};

/// A stream that might be encrypted with TLS.
#[derive(Debug)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum MaybeTlsStream<S: Read + Write> {
    /// Unencrypted stream.
    Plain(S),

    /// Stream encrypted with [`rustls`].
    #[cfg(feature = "rustls")]
    Rustls(rustls::StreamOwned<rustls::ClientConnection, S>),

    /// Stream encrypted with [`native_tls`].
    #[cfg(feature = "native-tls")]
    NativeTls(native_tls::TlsStream<S>),
}

impl<S: Read + Write> MaybeTlsStream<S> {
    /// Creates an unencrypted stream.
    pub fn plain(stream: impl Into<S>) -> Self {
        Self::Plain(stream.into())
    }

    /// Create an encrypted stream.
    pub fn tls(stream: impl Into<Self>) -> Self {
        stream.into()
    }
}

#[cfg(feature = "rustls")]
impl<S: Read + Write> From<rustls::StreamOwned<rustls::ClientConnection, S>> for MaybeTlsStream<S> {
    fn from(stream: rustls::StreamOwned<rustls::ClientConnection, S>) -> Self {
        Self::Rustls(stream)
    }
}

#[cfg(feature = "native-tls")]
impl<S: Read + Write> From<native_tls::TlsStream<S>> for MaybeTlsStream<S> {
    fn from(stream: native_tls::TlsStream<S>) -> Self {
        Self::NativeTls(stream)
    }
}

impl<S: Read + Write> Read for MaybeTlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Plain(stream) => stream.read(buf),
            #[cfg(feature = "rustls")]
            Self::Rustls(stream) => stream.read(buf),
            #[cfg(feature = "native-tls")]
            Self::NativeTls(stream) => stream.read(buf),
        }
    }
}

impl<S: Read + Write> Write for MaybeTlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            Self::Plain(stream) => stream.write(buf),
            #[cfg(feature = "rustls")]
            Self::Rustls(stream) => stream.write(buf),
            #[cfg(feature = "native-tls")]
            Self::NativeTls(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            Self::Plain(stream) => stream.flush(),
            #[cfg(feature = "rustls")]
            Self::Rustls(stream) => stream.flush(),
            #[cfg(feature = "native-tls")]
            Self::NativeTls(stream) => stream.flush(),
        }
    }
}
