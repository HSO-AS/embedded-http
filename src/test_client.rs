extern crate std;

use std::io::Write;
use std::prelude::rust_2021::*;

use httptest::{Server};

use std::io::Read;

pub struct TestClient {
    inner: std::net::TcpStream,
}

impl TestClient {
    pub fn new(host: &Server) -> Self {
        Self { inner: std::net::TcpStream::connect(host.addr()).unwrap() }
    }

    pub fn send(&mut self, req: &[u8]) -> Vec<u8> {
        self.inner.write_all(req).unwrap();

        let mut resp = Vec::new();

        while crate::response::Response::new(resp.as_slice()).check().is_err() {
            let mut buf = [0; 512];
            let num = self.inner.read(&mut buf).unwrap();
            resp.extend_from_slice(&buf[..num]);
        }

        resp
    }
}