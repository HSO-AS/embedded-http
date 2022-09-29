use core::str::FromStr;
use core::str::from_utf8;

pub struct Response<'a> {
    inner: &'a [u8],

    /// used to lazy evaluate status code
    status_code: Option<u16>,

    /// used to lazy evaluate content_length
    content_length: Option<usize>,
}

impl<'a> Response<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        Self { inner: content, status_code: None, content_length: None }
    }

    /// Extract the status code from the response
    /// returns None if no status code is found
    pub fn status_code(&mut self) -> Option<u16> {
        if let Some(sc) = self.status_code {
            return Some(sc);
        }
        let mut it = self.inner.split(|v| v == &b'\r');
        let line = it.next()?;
        let line = from_utf8(line).ok()?;
        let start_idx = line.find("HTTP/1.1 ")? + "HTTP/1.1 ".len();
        self.status_code = u16::from_str(&line[start_idx..start_idx + 3]).ok();
        self.status_code
    }

    /// Extract the content length from the response
    /// returns None if no content length is found
    pub fn content_length(&mut self) -> Option<usize> {
        if let Some(cl) = self.content_length {
            return Some(cl);
        }
        let it = self.inner.split(|v| v == &b'\r');
        for line in it {
            let line = from_utf8(line).ok()?;
            if let Some(start_idx) = line.find("content-length: ") {
                self.content_length = usize::from_str(&line[start_idx + "content-length: ".len()..line.len()]).ok();
                return self.content_length;
            }
        }
        None
    }

    /// Extract the body of the response
    /// returns None if no content length is found
    /// returns empty slice if content length is 0
    pub fn body(&mut self) -> Option<&'a [u8]> {
        let content_length = self.content_length()?;
        Some(&self.inner[self.inner.len() - content_length..])
    }

    /// Extract the header of the response
    /// returns None if no content length is found or header is invalid utf8
    pub fn header(&mut self) -> Option<&'a str> {
        let content_length = self.content_length()?;
        from_utf8(&self.inner[..self.inner.len() - content_length]).ok()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::{vec::Vec, string::String};
    use crate::writer::slice_writer::SliceWriter;

    const SIMPLE_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\nconnection: close\r\ndate: Wed, 28 Sep 2022 08:23:31 GMT\r\n\r\n";
    const BODY_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\ncontent-length: 132\r\nvary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers\r\ncontent-type: application/json\r\ndate: Wed, 28 Sep 2022 09:00:53 GMT\r\n\r\n{\"status_code\":200,\"canonical_reason\":\"OK\",\"data\":\"tap.it backend built with rustc version 1.63.0 at 2022-09-05\",\"description\":null}";

    #[test]
    fn deserialize_simple() {
        let mut resp = Response::new(SIMPLE_RESPONSE);
        assert_eq!(resp.status_code().unwrap(), 200);

        assert_eq!(resp.content_length().unwrap(), 0);

        println!("header: {}", resp.header().unwrap());
        println!("body: {}", from_utf8(resp.body().unwrap()).unwrap());

        assert_eq!(resp.body().unwrap().len(), 0);
    }

    #[test]
    fn deserialize_body() {
        let mut resp = Response::new(BODY_RESPONSE);
        let header = resp.header().unwrap();
        let body = resp.body().unwrap();

        assert_eq!(resp.status_code().unwrap(), 200);

        assert_eq!(resp.content_length().unwrap(), 132);


        println!("header: {}", header);
        println!("body: {}", from_utf8(body).unwrap());

        println!("status_code: {}", resp.status_code().unwrap())
    }
}
