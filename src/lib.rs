//! Convenience wrapper for streams that allows to choose between plain and TLS streams
//! at runtime.
//!
//! A stream can be any type that implements [`AsyncRead`], [`AsyncWrite`] and [`Unpin`].

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "async-std")]
pub mod async_std;
#[cfg(feature = "std")]
pub mod std;
#[cfg(feature = "tokio")]
pub mod tokio;
