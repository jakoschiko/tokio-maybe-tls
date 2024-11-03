#![cfg(feature = "std")]

use std::{
    env,
    io::{stdin, Read, Write},
    net::TcpStream,
    sync::Arc,
};

use byte_string::ByteStr;
use rustls_platform_verifier::ConfigVerifierExt;
use tokio_maybe_tls::std::MaybeTlsStream;

fn main() {
    let host = env::var("HOST").unwrap_or(String::from("www.rust-lang.org"));
    println!("This example will connect to {host}");

    #[allow(unused_mut)]
    let mut stream: MaybeTlsStream<TcpStream> = loop {
        println!("\nPlease enter plain|native-tls|rustls:");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "plain" => {
                let addr = format!("{host}:80");
                let tcp_stream = TcpStream::connect(addr).unwrap();
                break MaybeTlsStream::from(tcp_stream);
            }
            "native-tls" => {
                let addr = format!("{host}:443");
                let tcp_stream = TcpStream::connect(&addr).unwrap();
                let connector = native_tls::TlsConnector::new().unwrap();
                let tls_stream = connector.connect(&host, tcp_stream).unwrap();
                break MaybeTlsStream::from(tls_stream);
            }
            "rustls" => {
                let addr = format!("{host}:443");
                let srv_name = host.clone().try_into().unwrap();
                let tcp_stream = TcpStream::connect(&addr).unwrap();
                let tls_config = Arc::new(rustls::ClientConfig::with_platform_verifier());
                let tls_connection = rustls::client::ClientConnection::new(tls_config, srv_name);
                let tls_stream = rustls::StreamOwned::new(tls_connection.unwrap(), tcp_stream);
                break MaybeTlsStream::from(tls_stream);
            }
            _ => continue,
        }
    };

    let content = format!("GET / HTTP/1.0\r\nHost: {host}\r\n\r\n");
    stream.write_all(content.as_bytes()).unwrap();
    println!("\nSent: {content:?}");

    let mut plaintext = Vec::new();
    stream.read_to_end(&mut plaintext).unwrap();
    println!("\nReceived: {:?}", ByteStr::new(&plaintext));
}
