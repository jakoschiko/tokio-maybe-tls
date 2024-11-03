#![cfg(feature = "futures")]

use std::{env, sync::Arc};

use async_std::{
    io::{stdin, ReadExt, WriteExt},
    net::TcpStream,
};
use byte_string::ByteStr;
use rustls_platform_verifier::ConfigVerifierExt;
use tokio_maybe_tls::{futures::Futures, MaybeTlsStream};

#[async_std::main]
async fn main() {
    let host = env::var("HOST").unwrap_or(String::from("www.rust-lang.org"));
    let host = host.as_str();

    println!("This example will connect to {host}");

    let mut stream: MaybeTlsStream<Futures<TcpStream>> = loop {
        println!("\nPlease enter plain|native-tls|rustls:");

        let mut input = String::new();
        stdin().read_line(&mut input).await.unwrap();

        match input.trim() {
            "plain" => {
                let tcp_stream = TcpStream::connect((host, 80)).await.unwrap();
                break MaybeTlsStream::plain(tcp_stream);
            }
            "native-tls" => {
                let tcp_stream = TcpStream::connect((host, 443)).await.unwrap();
                let connector = async_native_tls::TlsConnector::new();
                let tls_stream = connector.connect(host, tcp_stream).await.unwrap();
                break MaybeTlsStream::futures_tls(tls_stream);
            }
            "rustls" => {
                let srv_name = host.to_owned().try_into().unwrap();
                let tcp_stream = TcpStream::connect((host, 443)).await.unwrap();
                let tls_config = Arc::new(rustls::ClientConfig::with_platform_verifier());
                let tls_connector = futures_rustls::TlsConnector::from(tls_config);
                let tls_stream = tls_connector.connect(srv_name, tcp_stream).await.unwrap();
                break MaybeTlsStream::futures_tls(tls_stream);
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
