#[cfg(any(feature = "async-std-rustls", feature = "async-std-native-tls"))]
use async_std::net::TcpStream;
use byte_string::ByteStr;
use futures::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(any(feature = "tokio-rustls", feature = "tokio-native-tls"))]
use tokio::net::TcpStream;
use tokio_maybe_tls::MaybeTlsStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[tokio::main]
async fn main() {
    let stdin = std::io::stdin();

    let host = "www.rust-lang.org";
    println!("This example will connect to {host}");

    #[allow(unused_mut)]
    let mut maybe_tls_stream: MaybeTlsStream<TcpStream> = loop {
        println!("\nPlease enter plain|tls:");

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "plain" => {
                let addr = format!("{host}:80");
                let tcp_stream = TcpStream::connect(addr).await.unwrap();
                break MaybeTlsStream::Plain(tcp_stream);
            }
            #[cfg(feature = "tokio-native-tls")]
            "tls" => {
                let addr = format!("{host}:443");
                let tcp_stream = TcpStream::connect(&addr).await.unwrap();
                let connector = tokio_native_tls::native_tls::TlsConnector::builder()
                    .build()
                    .unwrap();
                let connector = tokio_native_tls::TlsConnector::from(connector);
                let tls_stream = connector.connect(host, tcp_stream).await.unwrap();
                break MaybeTlsStream::from(tls_stream);
            }
            #[cfg(feature = "tokio-rustls")]
            "tls" => {
                use std::sync::Arc;

                let mut root_store = tokio_rustls::rustls::RootCertStore::empty();
                for cert in rustls_native_certs::load_native_certs().unwrap() {
                    root_store.add(cert).unwrap();
                }
                let config = tokio_rustls::rustls::ClientConfig::builder()
                    .with_root_certificates(root_store)
                    .with_no_client_auth();
                let connector = tokio_rustls::TlsConnector::from(Arc::new(config));
                let dnsname = tokio_rustls::rustls::pki_types::ServerName::try_from(host).unwrap();
                let addr = format!("{host}:443");
                let tcp_stream = TcpStream::connect(addr).await.unwrap();
                let tls_stream = connector.connect(dnsname, tcp_stream).await.unwrap();
                break MaybeTlsStream::from(tls_stream);
            }
            #[cfg(feature = "async-std-native-tls")]
            "tls" => {
                let addr = format!("{host}:443");
                let tcp_stream = TcpStream::connect(&addr).await.unwrap();
                let tls_stream = async_native_tls::connect(host, tcp_stream).await.unwrap();
                break MaybeTlsStream::from(tls_stream);
            }
            #[cfg(feature = "async-std-rustls")]
            "tls" => {
                let addr = format!("{host}:443");
                let tcp_stream = TcpStream::connect(&addr).await.unwrap();
                let connector = async_tls::TlsConnector::default();
                let tls_stream = connector.connect(host, tcp_stream).await.unwrap();
                break MaybeTlsStream::from(tls_stream);
            }
            _ => (),
        }
    };

    #[cfg(any(feature = "tokio-native-tls", feature = "tokio-rustls"))]
    let mut maybe_tls_stream = maybe_tls_stream.compat();

    let content = format!("GET / HTTP/1.0\r\nHost: {host}\r\n\r\n");
    maybe_tls_stream
        .write_all(content.as_bytes())
        .await
        .unwrap();
    println!("\nSent: {content:?}");

    let mut plaintext = Vec::new();
    maybe_tls_stream.read_to_end(&mut plaintext).await.unwrap();
    println!("\nReceived: {:?}", ByteStr::new(&plaintext));
}
