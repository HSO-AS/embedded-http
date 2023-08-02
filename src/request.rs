use core::fmt::{Display, Formatter};

use core::write;
use embedded_io::blocking::Write;
use embedded_io::Error as IoError;

use crate::Error;

pub enum Method {
    Get,
    Put,
    Post,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Method::Get => {
                write!(f, "GET")
            }
            Method::Put => {
                write!(f, "PUT")
            }
            Method::Post => {
                write!(f, "POST")
            }
        }
    }
}

pub struct Request<'a, const D: usize = 8> {
    pub method: Method,
    pub path: &'a str,
    pub headers: [(&'a str, &'a str); D],
    header_len: usize,
    // pub body: Option<&'a T>,
}

impl<'a, const D: usize> Request<'a, D> {
    pub fn new(host: &'a str, path: &'a str) -> Result<Self, Error> {
        let mut req = Self {
            method: Method::Get,
            path,
            headers: [("", ""); D],
            header_len: 0,
        };
        req = req.insert_header(("Host", host))?;
        Ok(req)
    }
    pub fn get(&mut self) -> &mut Self {
        self.method = Method::Get;
        self
    }

    pub fn post(&mut self) -> &mut Self {
        self.method = Method::Post;
        self
    }

    pub fn put(&mut self) -> &mut Self {
        self.method = Method::Put;
        self
    }

    pub fn insert_header(mut self, header: (&'a str, &'a str)) -> Result<Self, Error> {
        if self.header_len == D {
            Err(Error::Other(""))
        } else {
            *self.headers.get_mut(self.header_len).unwrap() = header;
            self.header_len += 1;
            Ok(self)
        }
    }

    pub fn set_json(self) -> Result<Self, Error> {
        self.insert_header(("Content-Type", "application/json"))
    }

    // pub fn body(&mut self, body: &'a T) -> &mut Self {
    //     self.body = Some(body);
    //     self
    // }

    fn build_header_no_body_inner<W: Write>(&self, mut buf: W) -> Result<(), Error> {
        write!(buf, "{} {} HTTP/1.1\r\n", self.method, self.path)?;

        for (key, value) in &self.headers[..self.header_len] {
            write!(buf, "{}: {}\r\n", key, value)?;
        }

        write!(buf, "User-Agent: rust\r\n")?;

        Ok(())
    }

    pub fn build_header_no_body<W: Write>(&self, mut buf: W) -> Result<(), Error> {
        self.build_header_no_body_inner(&mut buf)?;

        write!(buf, "\r\n")?;
        Ok(())
    }
}

impl<'a, const D: usize> Request<'a, D> {
    pub fn build<W: Write>(self, body: &'_ [u8], mut buf: W) -> Result<(), Error> {
        self.build_header_no_body_inner(&mut buf)?;

        write!(buf, "Content-Length: {}\r\n\r\n", body.len())?;
        buf.write(body).map_err(|e| Error::from(e.kind()))?;

        Ok(())
    }
}

#[cfg(all(feature = "serde_json", feature = "alloc"))]
impl<'a, const D: usize> Request<'a, D> {
    pub fn build_json<W: Write, T: Serialize>(mut self, body: T, buf: W) -> Result<(), Error> {
        self = self.set_json()?;
        let body_ser = serde_json::to_string(&body)?;

        self.build(body_ser.as_bytes(), buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;

    use core::str::from_utf8;

    #[test]
    fn build_simple() {
        let mut req: Request = Request::new("api.aqsense.no", "/v1/health").unwrap();
        req.get();

        let mut buf = Vec::new();

        req.build_header_no_body(&mut buf).unwrap();

        println!("{}", from_utf8(buf.as_slice()).unwrap());
    }

    #[test]
    fn build_simple_body() {
        let mut req: Request = Request::new("google.com", "/").unwrap();
        let body = "hei";
        req.post();

        let mut buf = Vec::new();

        req.build(body.as_bytes(), &mut buf).unwrap();

        println!("{}", from_utf8(buf.as_slice()).unwrap());
    }

    #[test]
    fn build_simple_body_no_alloc() {
        let mut req: Request = Request::new("google.com", "/").unwrap();
        let body = "hei";
        req.post();

        let mut buf = [0; 512];

        req.build(body.as_bytes(), buf.as_mut()).unwrap();

        // Find the first 0 to find end of string
        let len = buf.iter().enumerate().find(|v| v.1 == &0).unwrap().0;

        println!("{}", from_utf8(&buf[0..len]).unwrap());
    }

    #[cfg(all(feature = "serde_json", feature = "alloc"))]
    #[test]
    fn build_json_body() {
        let mut req: Request = Request::new("google.com", "/").unwrap();
        let body = serde_json::json!({"hei": "hade"});
        req.post();

        let mut buf = Vec::new();

        req.build_json(body, &mut buf).unwrap();

        println!("{}", from_utf8(buf.as_slice()).unwrap());
    }
}
