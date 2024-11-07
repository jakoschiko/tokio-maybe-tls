//! # Maybe TLS stream
//!
//! Convenience wrapper for streams that allows to choose between
//! plain and TLS streams at runtime.

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(any(feature = "futures-rustls", feature = "async-native-tls"))]
pub mod async_std;
#[cfg(any(feature = "rustls", feature = "native-tls"))]
pub mod std;
#[cfg(any(feature = "tokio-rustls", feature = "tokio-native-tls"))]
pub mod tokio;
