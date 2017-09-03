extern crate native_tls;
extern crate reqwest;
use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

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

fn reqwest_get(url: &str, insecure: bool) {
    let mut body_data = vec![];
    let mut body: String = String::new();

    println!("Start reqwest");
    let mut clientbuilder = reqwest::ClientBuilder::new().unwrap();

    if insecure {
        clientbuilder.danger_disable_certificate_validation_entirely();
    }
    let client = clientbuilder.build().unwrap();

    println!("Have client");
    let mut res = client.get(url).unwrap().send().unwrap();
    println!("Have response");
    let _ = res.read_to_end(&mut body_data).unwrap();
    let body = String::from_utf8_lossy(&body_data);
    println!("{}", body);
}

fn main() {
    reqwest_get("https://icanhazip.com", false);
    reqwest_get("https://expired.badssl.com/", true);
    reqwest_get("https://wrong.host.badssl.com/", true);
    reqwest_get("https://self-signed.badssl.com/", true);
    reqwest_get("https://untrusted-root.badssl.com/", true);
    reqwest_get("https://revoked.badssl.com/", true);
    reqwest_get("https://pinning-test.badssl.com/", true);
}
