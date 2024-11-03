#![cfg(feature = "tokio")]

use std::{env, sync::Arc};

use byte_string::ByteStr;
use rustls_platform_verifier::ConfigVerifierExt;
use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_maybe_tls::tokio::MaybeTlsStream;

#[tokio::main]
async fn main() {
    let host = env::var("HOST").unwrap_or(String::from("www.rust-lang.org"));
    println!("This example will connect to {host}");

    #[allow(unused_mut)]
    let mut stream: MaybeTlsStream<TcpStream> = loop {
        println!("\nPlease enter plain|native-tls|rustls:");

        let mut input = String::new();
        BufReader::new(stdin()).read_line(&mut input).await.unwrap();

        match input.trim() {
            "plain" => {
                let addr = format!("{host}:80");
                let tcp_stream = TcpStream::connect(addr).await.unwrap();
                break MaybeTlsStream::from(tcp_stream);
            }
            "native-tls" => {
                let addr = format!("{host}:443");
                let tcp_stream = TcpStream::connect(&addr).await.unwrap();
                let connector = tokio_native_tls::native_tls::TlsConnector::new().unwrap();
                let connector = tokio_native_tls::TlsConnector::from(connector);
                let tls_stream = connector.connect(&host, tcp_stream).await.unwrap();
                break MaybeTlsStream::from(tls_stream);
            }
            "rustls" => {
                let addr = format!("{host}:443");
                let srv_name = host.clone().try_into().unwrap();
                let tcp_stream = TcpStream::connect(&addr).await.unwrap();
                let tls_config = Arc::new(rustls::ClientConfig::with_platform_verifier());
                let tls_connector = tokio_rustls::TlsConnector::from(tls_config);
                let tls_stream = tls_connector.connect(srv_name, tcp_stream).await.unwrap();
                break MaybeTlsStream::from(tls_stream);
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
