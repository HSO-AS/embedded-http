use core::str::{FromStr, Utf8Error};
use core::str::from_utf8;
use core::num::ParseIntError;

#[cfg(all(feature = "alloc", feature = "defmt"))]
use alloc::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseError {
    Utf8Error(Utf8Error),
    ParseIntError(ParseIntError),
    HeaderNotFound,
    Error,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ResponseError {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            ResponseError::Utf8Error(e) => {
                #[cfg(not(feature = "alloc"))]
                defmt::write!(fmt, "Utf8Error()");

                #[cfg(feature = "alloc")]
                defmt::write!(fmt, "Utf8Error({})", e.to_string());
            }
            ResponseError::ParseIntError(e) => {
                #[cfg(not(feature = "alloc"))]
                defmt::write!(fmt, "ParseIntError()");

                #[cfg(feature = "alloc")]
                defmt::write!(fmt, "ParseIntError({})", e.to_string());
            }
            ResponseError::HeaderNotFound => {
                defmt::write!(fmt, "HeaderNotFound");
            }
            ResponseError::Error => {
                defmt::write!(fmt, "Error");
            }
        }
    }
}

impl From<Utf8Error> for ResponseError {
    fn from(e: Utf8Error) -> Self {
        ResponseError::Utf8Error(e)
    }
}

impl From<ParseIntError> for ResponseError {
    fn from(e: ParseIntError) -> Self {
        ResponseError::ParseIntError(e)
    }
}

type Result<T> = core::result::Result<T, ResponseError>;

pub struct Response<'a> {
    inner: &'a [u8],

    /// used to lazy evaluate status code
    status_code: Option<u16>,

    /// used to lazy evaluate content_length
    content_length: Option<usize>,

    /// used to lazy evaluate header_length
    header_length: Option<usize>,
}

impl<'a> Response<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        Self { inner: content, status_code: None, content_length: None, header_length: None }
    }

    /// Creates a response, and checks that the header_len + content_len = buffer.len()
    pub fn new_checked(content: &'a [u8]) -> Result<Self> {
        let mut resp = Self::new(content);
        if resp.header_len()? + resp.content_length()? == content.len() {
            resp.status_code()?;
            Ok(resp)
        } else {
            Err(ResponseError::Error)
        }
    }

    /// Calculate header len
    pub fn header_len(&mut self) -> Result<usize> {
        if let Some(hl) = self.header_length {
            return Ok(hl);
        }
        let mut num_bytes = 0;
        for line in self.inner.split(|v| v == &b'\n') {
            num_bytes += line.len() + 1;
            if line == b"\r" {
                break;
            }
        }
        self.header_length = Some(num_bytes);
        Ok(num_bytes)
    }

    /// Extract the status code from the response
    /// returns None if no status code is found
    pub fn status_code(&mut self) -> Result<u16> {
        if let Some(sc) = self.status_code {
            return Ok(sc);
        }
        let mut it = self.inner.split(|v| v == &b'\n');
        let line = it.next().ok_or_else(|| ResponseError::Error)?;
        let line = from_utf8(line)?;
        let start_idx = line.find("HTTP/1.1 ").ok_or_else(|| ResponseError::HeaderNotFound)? + "HTTP/1.1 ".len();
        let status_code = u16::from_str(&line[start_idx..start_idx + 3])?;
        self.status_code = Some(status_code);
        Ok(status_code)
    }

    /// Extract the content length from the response
    /// returns None if no content length is found
    pub fn content_length(&mut self) -> Result<usize> {
        if let Some(cl) = self.content_length {
            return Ok(cl);
        }
        let it = self.inner.split(|v| v == &b'\n');
        for line in it {
            let line = from_utf8(line)?;
            if let Some(start_idx) = line.find("content-length: ") {
                let cl = usize::from_str(&line[start_idx + "content-length: ".len()..line.len() - 1])?;
                self.content_length = Some(cl);
                return Ok(cl);
            }
        }
        Err(ResponseError::HeaderNotFound)
    }


    /// Extract the body of the response
    /// returns None if no content length is found
    /// returns empty slice if content length is 0
    pub fn body(&mut self) -> Result<&'a [u8]> {
        Ok(&self.inner[self.header_len()?..self.header_len()? + self.content_length()?])
    }

    /// Extract the body of the response and parses as str
    /// returns None if no content length is found
    /// returns empty slice if content length is 0
    pub fn body_as_str(&mut self) -> Result<&'a str> {
        Ok(from_utf8(self.body()?)?)
    }

    /// Extract the header of the response
    /// returns None if no content length is found or header is invalid utf8
    pub fn header(&mut self) -> Result<&'a str> {
        Ok(from_utf8(&self.inner[..self.header_len()?])?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn checked_deserialize_body() {
        let mut resp = Response::new(BODY_RESPONSE);
        dbg!(resp.header_len().unwrap());
        dbg!(resp.content_length().unwrap());
        dbg!(BODY_RESPONSE.len());
        let mut resp = Response::new_checked(BODY_RESPONSE).unwrap();
        let header = resp.header().unwrap();
        let body = resp.body().unwrap();

        assert_eq!(resp.status_code().unwrap(), 200);

        assert_eq!(resp.content_length().unwrap(), 132);


        println!("header: {}", header);
        println!("body: {}", from_utf8(body).unwrap());

        println!("status_code: {}", resp.status_code().unwrap())
    }
}
