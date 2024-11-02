//! Convenience wrapper for streams that allows to choose between plain and TLS streams
//! at runtime.
//!
//! A stream can be any type that implements [`AsyncRead`], [`AsyncWrite`] and [`Unpin`].

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(any(feature = "async-std-native-tls", feature = "async-std-rustls"))]
#[path = "async_std.rs"]
mod runtime;
#[cfg(any(feature = "tokio-native-tls", feature = "tokio-rustls"))]
#[path = "tokio.rs"]
mod runtime;

/// A stream that might be encrypted with TLS.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum MaybeTlsStream<S> {
    /// Unencrypted stream.
    Plain(S),
    /// Encrypted stream.
    Tls(runtime::TlsStream<S>),
}
