#![cfg(feature = "std")]

use std::{
    env,
    io::{stdin, Read, Write},
    net::TcpStream,
    sync::Arc,
};

use byte_string::ByteStr;
use rustls_platform_verifier::ConfigVerifierExt;
use tokio_maybe_tls::{std::Std, MaybeTlsStream};

fn main() {
    let host = env::var("HOST").unwrap_or(String::from("www.rust-lang.org"));
    let host = host.as_str();

    println!("This example will connect to {host}");

    let mut stream: MaybeTlsStream<Std<TcpStream>> = loop {
        println!("\nPlease enter plain|native-tls|rustls:");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "plain" => {
                let tcp_stream = TcpStream::connect((host, 80)).unwrap();
                break MaybeTlsStream::plain(tcp_stream);
            }
            "native-tls" => {
                let tcp_stream = TcpStream::connect((host, 443)).unwrap();
                let connector = native_tls::TlsConnector::new().unwrap();
                let tls_stream = connector.connect(host, tcp_stream).unwrap();
                break MaybeTlsStream::std_tls(tls_stream);
            }
            "rustls" => {
                let srv_name = host.to_owned().try_into().unwrap();
                let tcp_stream = TcpStream::connect((host, 443)).unwrap();
                let tls_config = Arc::new(rustls::ClientConfig::with_platform_verifier());
                let tls_connection = rustls::client::ClientConnection::new(tls_config, srv_name);
                let tls_stream = rustls::StreamOwned::new(tls_connection.unwrap(), tcp_stream);
                break MaybeTlsStream::std_tls(tls_stream);
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
