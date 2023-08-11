use core::write;
use embedded_io::Write;

use alloc::vec::Vec;

use crate::{Error, Result};

#[allow(unused_imports)]
use crate::prelude::*;

use http::{HeaderName, HeaderValue, Request};
use http::uri::PathAndQuery;

static USER_AGENT: HeaderValue = HeaderValue::from_static(":)");


pub struct RequestWrapper<T> {
    pub inner: Request<T>,
}

impl<T> From<Request<T>> for RequestWrapper<T> {
    fn from(inner: Request<T>) -> Self {
        Self { inner }
    }
}


impl<T> RequestWrapper<T> {
    pub fn new(request: Request<T>) -> Self {
        Self {
            inner: request,
        }
    }

    fn write_header<W: Write>(&self, mut w: W, extra_headers: &[(&HeaderName, &HeaderValue)]) -> Result<(), Error> {
        fn write_header_value<W: Write>(name: &HeaderName, value: &HeaderValue, w: &mut W) -> Result<()> {
            write!(w, "{}: ", name)?;
            w.write_all(value.as_bytes())?;
            write!(w, "\r\n")?;
            Ok(())
        }

        write!(w, "{} {} HTTP/1.1\r\n", self.inner.method(), self.inner.uri().path_and_query()
            .unwrap_or(&PathAndQuery::from_static("/")))?;

        // write host field
        write_header_value(
            &http::header::HOST,
            &HeaderValue::from_str(self.inner.uri().host().unwrap_or("")).unwrap(),
            &mut w)?;

        // write user agent field
        write_header_value(
            &http::header::USER_AGENT,
            &USER_AGENT,
            &mut w)?;

        for (name, value) in
        self.inner.headers().iter()
            .filter(|(key, _)| key.ne(&http::header::USER_AGENT))
            .filter(|(key, _)| key.ne(&http::header::HOST))
        {
            write_header_value(name, value, &mut w)?;
        }

        for (name, value) in extra_headers {
            write_header_value(name, value, &mut w)?;
        }

        write!(w, "\r\n")?;

        Ok(())
    }
}

#[cfg(feature = "serde_json")]
impl<T: Serialize> RequestWrapper<T> {
    pub fn write_json_to<W: Write>(&self, mut w: W) -> Result<()> {
        let body = serde_json::to_string(&self.inner.body())?;

        self.write_header(&mut w, &[
            (&http::header::CONTENT_TYPE, &HeaderValue::from_static(crate::mime::APPLICATION_JSON)),
            (&http::header::CONTENT_LENGTH, &HeaderValue::from(body.len())),
        ])?;

        w.write_all(body.as_bytes())?;


        Ok(())
    }

    pub fn to_json_vec(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_json_to(&mut buf)?;
        Ok(buf)
    }
}


impl<T: ToRequestBody> RequestWrapper<T> {
    pub fn write_to<W: Write>(&self, mut w: W) -> Result<()> {
        // If there is no content type, we can just write the header and be done
        let ct = if let Some(ct) = self.inner.body().content_type() {
            ct
        } else {
            self.write_header(&mut w, &[])?;
            return Ok(());
        };

        let mut body = None;

        // If the content length is known, we can write the body directly to the writer
        let cl = if let Some(cl) = self.inner.body().content_length() {
            cl
        } else {
            let mut body_inner = Vec::new();
            self.inner.body().write_body(&mut body_inner)?;
            let cl = HeaderValue::from(body_inner.len());
            body = Some(body_inner);
            cl
        };

        self.write_header(&mut w, &[
            (&http::header::CONTENT_TYPE, &ct),
            (&http::header::CONTENT_LENGTH, &cl),
        ])?;

        if let Some(b) = body {
            w.write_all(&b)?;
        } else {
            self.inner.body().write_body(&mut w)?;
        }

        Ok(())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_to(&mut buf)?;
        Ok(buf)
    }
}

pub trait ToRequestBody {
    fn write_body<W: Write>(&self, w: W) -> Result<()>;

    fn content_type(&self) -> Option<HeaderValue>;

    fn content_length(&self) -> Option<HeaderValue> {
        None
    }
}

impl<B: ToRequestBody> ToRequestBody for &B {
    fn write_body<W: Write>(&self, w: W) -> Result<()> {
        (*self).write_body(w)
    }

    fn content_type(&self) -> Option<HeaderValue> {
        (*self).content_type()
    }

    fn content_length(&self) -> Option<HeaderValue> {
        (*self).content_length()
    }
}

impl ToRequestBody for () {
    fn write_body<W: Write>(&self, _w: W) -> Result<()> {
        Ok(())
    }

    fn content_type(&self) -> Option<HeaderValue> {
        None
    }
}

impl<'a> ToRequestBody for &'a str {
    fn write_body<W: Write>(&self, mut w: W) -> Result<()> {
        Ok(w.write_all(self.as_bytes())?)
    }

    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static(crate::mime::TEXT_PLAIN_UTF_8))
    }

    fn content_length(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from(self.len()))
    }
}

impl<'a> ToRequestBody for &'a [u8] {
    fn write_body<W: Write>(&self, mut w: W) -> Result<()> {
        Ok(w.write_all(self)?)
    }

    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static(crate::mime::APPLICATION_OCTET_STREAM))
    }

    fn content_length(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from(self.len()))
    }
}

/*
impl<T> Request<'a, D> {
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

        write!(buf, "User-Agent: {USER_AGENT}\r\n")?;

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
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;

    use core::str::from_utf8;

    #[test]
    fn build_no_body() {
        let request = Request::get("https://api.aqsense.no/v1/health").body(()).unwrap();
        let req = RequestWrapper::from(request);

        let mut buf = Vec::new();

        req.write_to(&mut buf).unwrap();

        println!("{}", from_utf8(buf.as_slice()).unwrap());


        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);

        let body_status = req.parse(buf.as_slice()).unwrap();

        // Check path, method and version
        assert_eq!(req.path.unwrap(), "/v1/health");
        assert_eq!(req.method.unwrap(), "GET");
        assert_eq!(req.version.unwrap(), 1);

        // check content type
        assert!(!req.headers.iter().any(|header| header.name == http::header::CONTENT_TYPE));

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        assert_eq!(buf[body_status.unwrap()..].len(), 0);
    }

    #[test]
    fn build_str_body() {
        let body = "hei";
        let req = Request::post("https://google.com/").body(body).unwrap();
        let req = RequestWrapper::from(req);

        let mut buf = Vec::new();
        req.write_to(&mut buf).unwrap();


        println!("{}", from_utf8(buf.as_slice()).unwrap());

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);

        let body_status = req.parse(buf.as_slice()).unwrap();

        // Check path, method and version
        assert_eq!(req.path.unwrap(), "/");
        assert_eq!(req.method.unwrap(), "POST");
        assert_eq!(req.version.unwrap(), 1);

        // check content type
        let ct = req.headers.iter().find(|header| header.name == http::header::CONTENT_TYPE).unwrap();
        assert_eq!(from_utf8(ct.value).unwrap(), crate::mime::TEXT_PLAIN_UTF_8);

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        assert_eq!(&buf[body_status.unwrap()..], body.as_bytes());
    }

    #[test]
    fn build_byte_body() {
        let body = b"hei";
        let req = Request::post("https://google.com/").body(body.as_slice()).unwrap();
        let req = RequestWrapper::from(req);

        let mut buf = Vec::new();
        req.write_to(&mut buf).unwrap();


        println!("{}", from_utf8(buf.as_slice()).unwrap());

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);

        let body_status = req.parse(buf.as_slice()).unwrap();

        // Check path, method and version
        assert_eq!(req.path.unwrap(), "/");
        assert_eq!(req.method.unwrap(), "POST");
        assert_eq!(req.version.unwrap(), 1);

        // check content type
        let ct = req.headers.iter().find(|header| header.name == http::header::CONTENT_TYPE).unwrap();
        assert_eq!(from_utf8(ct.value).unwrap(), crate::mime::APPLICATION_OCTET_STREAM);

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        assert_eq!(&buf[body_status.unwrap()..], body);
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn build_json_body() {
        let body = "hei";
        let req = Request::post("https://google.com/").body(body).unwrap();
        let req = RequestWrapper::from(req);

        let mut buf = Vec::new();
        req.write_json_to(&mut buf).unwrap();


        println!("{}", from_utf8(buf.as_slice()).unwrap());

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);

        let body_status = req.parse(buf.as_slice()).unwrap();

        // Check path, method and version
        assert_eq!(req.path.unwrap(), "/");
        assert_eq!(req.method.unwrap(), "POST");
        assert_eq!(req.version.unwrap(), 1);

        // check content type
        let ct = req.headers.iter().find(|header| header.name == http::header::CONTENT_TYPE).unwrap();
        assert_eq!(from_utf8(ct.value).unwrap(), crate::mime::APPLICATION_JSON);

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        let recv_body: serde_json::Value = serde_json::from_slice(&buf[body_status.unwrap()..]).unwrap();
        assert_eq!(recv_body, body);
    }

    #[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
    #[cfg_attr(feature = "serde_json", derive(serde_derive::Serialize, serde_derive::Deserialize))]
    #[repr(packed)]
    struct TestStruct {
        a: u32,
        b: u32,
    }

    impl AsRef<[u8]> for TestStruct {
        fn as_ref(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, std::mem::size_of::<Self>()) }
        }
    }

    impl AsMut<[u8]> for TestStruct {
        fn as_mut(&mut self) -> &mut [u8] {
            unsafe { std::slice::from_raw_parts_mut(self as *mut _ as *mut u8, std::mem::size_of::<Self>()) }
        }
    }

    impl ToRequestBody for TestStruct {
        fn write_body<W: Write>(&self, mut w: W) -> Result<()> {
            Ok(w.write_all(self.as_ref())?)
        }

        fn content_type(&self) -> Option<HeaderValue> {
            Some(HeaderValue::from_static("application/test"))
        }

        fn content_length(&self) -> Option<HeaderValue> {
            Some(HeaderValue::from_static("8"))
        }
    }

    #[test]
    fn build_custom() {
        let body = TestStruct { a: 1, b: 2 };
        let req = http::Request::post("https://google.com/").body(&body).unwrap();
        let req = RequestWrapper::from(req);

        let mut buf = Vec::new();
        req.write_to(&mut buf).unwrap();


        println!("{}", from_utf8(buf.as_slice()).unwrap());

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);

        let body_status = req.parse(buf.as_slice()).unwrap();

        // Check path, method and version
        assert_eq!(req.path.unwrap(), "/");
        assert_eq!(req.method.unwrap(), "POST");
        assert_eq!(req.version.unwrap(), 1);

        // check content type
        let ct = req.headers.iter().find(|header| header.name == http::header::CONTENT_TYPE).unwrap();
        assert_eq!(from_utf8(ct.value).unwrap(), "application/test");

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        let mut new_body = TestStruct::default();

        assert_eq!(&buf[body_status.unwrap()..], body.as_ref());

        new_body.as_mut().copy_from_slice(&buf[body_status.unwrap()..]);
        assert_eq!(new_body, body);
    }


    #[cfg(feature = "serde_json")]
    #[test]
    fn build_custom_json() {
        let body = TestStruct { a: 1, b: 2 };
        let req = http::Request::post("https://google.com/").body(&body).unwrap();
        let req = RequestWrapper::from(req);

        let mut buf = Vec::new();
        req.write_json_to(&mut buf).unwrap();


        println!("{}", from_utf8(buf.as_slice()).unwrap());

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);

        let body_status = req.parse(buf.as_slice()).unwrap();

        // Check path, method and version
        assert_eq!(req.path.unwrap(), "/");
        assert_eq!(req.method.unwrap(), "POST");
        assert_eq!(req.version.unwrap(), 1);

        // check content type
        let ct = req.headers.iter().find(|header| header.name == http::header::CONTENT_TYPE).unwrap();
        assert_eq!(from_utf8(ct.value).unwrap(), crate::mime::APPLICATION_JSON);

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        let new_body: TestStruct = serde_json::from_slice(&buf[body_status.unwrap()..]).unwrap();

        assert_eq!(new_body, body);
    }
}
