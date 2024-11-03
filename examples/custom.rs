#![cfg(feature = "tokio")]

use std::{env, sync::Arc};

use byte_string::ByteStr;
use rustls_platform_verifier::ConfigVerifierExt;
use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_maybe_tls::{MaybeTlsStream, Stream};
use tokio_rustls::client::TlsStream;

#[tokio::main]
async fn main() {
    let host = env::var("HOST").unwrap_or(String::from("www.rust-lang.org"));
    let host = host.as_str();

    println!("This example will connect to {host}");

    struct TokioRustlsStream;

    impl Stream for TokioRustlsStream {
        type Plain = TcpStream;
        type Tls = TlsStream<Self::Plain>;
    }

    let mut stream: MaybeTlsStream<TokioRustlsStream> = loop {
        println!("\nEnable TLS yes|no?");

        let mut input = String::new();
        BufReader::new(stdin()).read_line(&mut input).await.unwrap();

        match input.trim() {
            "no" => {
                let tcp_stream = TcpStream::connect((host, 80)).await.unwrap();
                break MaybeTlsStream::plain(tcp_stream);
            }
            "yes" => {
                let srv_name = host.to_owned().try_into().unwrap();
                let tcp_stream = TcpStream::connect((host, 443)).await.unwrap();
                let tls_config = Arc::new(rustls::ClientConfig::with_platform_verifier());
                let tls_connector = tokio_rustls::TlsConnector::from(tls_config);
                let tls_stream = tls_connector.connect(srv_name, tcp_stream).await.unwrap();
                break MaybeTlsStream::tls(tls_stream);
            }
            _ => continue,
        }
    };

    let content = format!("GET / HTTP/1.0\r\nHost: {host}\r\n\r\n");
    stream.write_all(content.as_bytes()).await.unwrap();
    println!("\nSent: {content:?}");

    let mut plaintext = Vec::new();
    stream.read_to_end(&mut plaintext).await.unwrap();
    println!("\nReceived: {:?}", ByteStr::new(&plaintext));
}
