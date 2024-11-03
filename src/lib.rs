//! # Maybe TLS stream
//!
//! Convenience wrapper for streams that allows to choose between
//! plain and TLS streams at runtime.

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "futures")]
pub mod futures;
#[cfg(feature = "std")]
pub mod std;
#[cfg(feature = "tokio")]
pub mod tokio;

/// A stream trait composed of a plain and TLS type.
pub trait Stream {
    type Plain;
    type Tls;
}

/// A stream that might be encrypted with TLS.
#[allow(clippy::large_enum_variant)]
pub enum MaybeTlsStream<S: Stream> {
    /// Unencrypted stream.
    Plain(S::Plain),

    /// Encrypted stream.
    Tls(S::Tls),
}

impl<S: Stream> MaybeTlsStream<S> {
    pub fn plain(stream: S::Plain) -> Self {
        Self::Plain(stream)
    }

    pub fn tls(stream: S::Tls) -> Self {
        Self::Tls(stream)
    }
}
