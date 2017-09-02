extern crate native_tls;
use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

use native_tls::backend::openssl::TlsConnectorBuilderExt;
fn connect_google() {
    let connector = TlsConnector::builder().unwrap().build().unwrap();

    let stream = TcpStream::connect("google.com:443").unwrap();
    let mut stream = connector.connect("google.com", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}

fn router() {
    let mut builder = TlsConnector::builder().unwrap();
    builder.danger_disable_certificate_validation_entirely();

    let connector = builder.build().unwrap();

    let stream = TcpStream::connect("10.0.0.1:443").unwrap();
    // let mut stream = connector.connect("google.com", stream).unwrap();
    let mut stream = connector.danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication( stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}

fn main() {
    router();
}
