use std::{
    io::{Read, Result, Write},
    net::TcpStream,
};

pub trait TlsStream<S: Read + Write>: Read + Write {}

impl<T: Read + Write, S: Read + Write> TlsStream<S> for T {}

/// A stream that might be encrypted with TLS.
#[allow(clippy::large_enum_variant)]
pub enum MaybeTlsStream<S> {
    /// Unencrypted stream.
    Plain(S),
    /// Encrypted stream.
    Tls(Box<dyn TlsStream<S>>),
}

impl<S: Read + Write> MaybeTlsStream<S> {
    pub fn plain(stream: S) -> Self {
        Self::Plain(stream)
    }

    pub fn tls(stream: Box<dyn TlsStream<S>>) -> Self {
        Self::Tls(stream)
    }
}

impl<S: Read + Write> Read for MaybeTlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Self::Plain(stream) => stream.read(buf),
            Self::Tls(stream) => stream.read(buf),
        }
    }
}

impl<S: Read + Write> Write for MaybeTlsStream<S> {
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

impl From<TcpStream> for MaybeTlsStream<TcpStream> {
    fn from(stream: TcpStream) -> Self {
        MaybeTlsStream::plain(stream)
    }
}

#[cfg(feature = "rustls")]
impl From<rustls::StreamOwned<rustls::ClientConnection, TcpStream>> for MaybeTlsStream<TcpStream> {
    fn from(stream: rustls::StreamOwned<rustls::ClientConnection, TcpStream>) -> Self {
        MaybeTlsStream::tls(Box::new(stream))
    }
}

#[cfg(feature = "native-tls")]
impl From<native_tls::TlsStream<TcpStream>> for MaybeTlsStream<TcpStream> {
    fn from(stream: native_tls::TlsStream<TcpStream>) -> Self {
        MaybeTlsStream::tls(Box::new(stream))
    }
}
