use core::write;
use embedded_io::Write;

use alloc::vec::Vec;

use crate::{Error, Result};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::header::{HeaderValue, HeaderKey};

use core::fmt::Display;

use crate::uri::Uri;

static USER_AGENT: HeaderValue<'static> = HeaderValue::from_static(b":)");


pub struct Request<'a, T> {
    pub header: Header<'a>,
    pub body: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Header<'a> {
    pub method: Method,
    pub uri: Uri<'a>,
    pub headers: Vec<(HeaderKey<'a>, HeaderValue<'a>)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

impl<'a> Header<'a> {
    pub fn into_owned(self) -> Header<'static> {
        Header {
            method: self.method,
            uri: self.uri.into_owned(),
            headers: self.headers.into_iter().map(|(k, v)| (k.into_owned(), v.into_owned())).collect(),
        }
    }

    pub fn into_borrowed<'b: 'a>(&'b self) -> Header<'b> {
        Header {
            method: self.method,
            uri: self.uri.into_borrowed(),
            headers: self.headers.iter().map(|(k, v)| (k.into_borrowed(), v.into_borrowed())).collect(),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.str())
    }
}

impl Method {
    pub fn str(&self) -> &'static str {
        match self {
            Method::Options => "OPTIONS",
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Trace => "TRACE",
            Method::Connect => "CONNECT",
            Method::Patch => "PATCH",
        }
    }
}

impl<'a, T> Request<'a, T> {
    pub fn new(method: Method, uri: Uri<'a>, body: T) -> Self {
        Self {
            header: Header {
                method,
                uri,
                headers: Vec::new(),
            },
            body,
        }
    }
}


impl<'a, T> Request<'a, T> {
    fn write_header<W: Write>(&self, mut w: W, extra_headers: &[(&HeaderKey, &HeaderValue)]) -> Result<(), Error> {
        fn write_header_value<W: Write>(name: &HeaderKey, value: &HeaderValue, w: &mut W) -> Result<()> {
            write!(w, "{}: ", name)?;
            w.write_all(value.as_ref())?;
            write!(w, "\r\n")?;
            Ok(())
        }

        write!(w, "{} {} HTTP/1.1\r\n", &self.header.method, self.header.uri.path_and_query)?;

        // write host field
        write_header_value(
            &crate::header::HOST,
            &self.header.uri.authority.as_ref().into(),
            &mut w)?;

        // write user agent field
        write_header_value(
            &crate::header::USER_AGENT,
            &USER_AGENT,
            &mut w)?;

        for (name, value) in
        self.header.headers.iter()
            .filter(|(key, _)| key.ne(&crate::header::USER_AGENT))
            .filter(|(key, _)| key.ne(&crate::header::HOST))
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
impl<'a, T: Serialize> Request<'a, T> {
    pub fn write_json_to<W: Write>(&self, mut w: W) -> Result<()> {
        let body = serde_json::to_string(&self.body)?;

        let mut b = itoa::Buffer::new();
        let cl = b.format(body.len());
        self.write_header(&mut w, &[
            (&crate::header::CONTENT_TYPE, &crate::mime::APPLICATION_JSON),
            (&crate::header::CONTENT_LENGTH, &cl.into()),
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


impl<'a, T: ToRequestBody> Request<'a, T> {
    pub fn write_to<W: Write>(&self, mut w: W) -> Result<()> {
        // If there is no content type, we can just write the header and be done
        let ct = if let Some(ct) = self.body.content_type() {
            ct
        } else {
            self.write_header(&mut w, &[])?;
            return Ok(());
        };

        let mut body = None;

        // If the content length is known, we can write the body directly to the writer
        let cl = if let Some(cl) = self.body.content_length() {
            cl
        } else {
            let mut body_inner = Vec::new();
            self.body.write_body(&mut body_inner)?;
            let cl = body_inner.len();
            body = Some(body_inner);
            cl
        };

        self.write_header(&mut w, &[
            (&crate::header::CONTENT_TYPE, &ct.into()),
            (&crate::header::CONTENT_LENGTH, &itoa::Buffer::new().format(cl).into()),
        ])?;

        if let Some(b) = body {
            w.write_all(&b)?;
        } else {
            self.body.write_body(&mut w)?;
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

    fn content_type<'a>(&'a self) -> Option<HeaderValue<'a>>;

    fn content_length(&self) -> Option<usize> {
        None
    }
}

impl<B: ToRequestBody> ToRequestBody for &B {
    fn write_body<W: Write>(&self, w: W) -> Result<()> {
        (*self).write_body(w)
    }

    fn content_type<'a>(&'a self) -> Option<HeaderValue<'a>> {
        (*self).content_type()
    }

    fn content_length(&self) -> Option<usize> {
        (*self).content_length()
    }
}

impl ToRequestBody for () {
    fn write_body<W: Write>(&self, _w: W) -> Result<()> {
        Ok(())
    }

    fn content_type<'a>(&'a self) -> Option<HeaderValue<'a>> {
        None
    }
}

impl<'body> ToRequestBody for &'body str {
    fn write_body<W: Write>(&self, mut w: W) -> Result<()> {
        Ok(w.write_all(self.as_bytes())?)
    }

    fn content_type<'a>(&'a self) -> Option<HeaderValue<'a>> {
        Some(crate::mime::TEXT_PLAIN_UTF_8)
    }

    fn content_length(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'body> ToRequestBody for &'body [u8] {
    fn write_body<W: Write>(&self, mut w: W) -> Result<()> {
        Ok(w.write_all(self)?)
    }

    fn content_type<'a>(&'a self) -> Option<HeaderValue<'a>> {
        Some(crate::mime::APPLICATION_OCTET_STREAM)
    }

    fn content_length(&self) -> Option<usize> {
        Some(self.len())
    }
}


pub struct RequestBuilder<'a> {
    headers: Vec<(HeaderKey<'a>, HeaderValue<'a>)>,
    method: Method,
    uri: Uri<'a>,
}

impl<'a> RequestBuilder<'a> {
    pub fn get(uri: &'a str) -> Result<Self> {
        Ok(Self {
            headers: Vec::new(),
            method: Method::Get,
            uri: Uri::parse(uri)?,
        })
    }

    pub fn post(uri: &'a str) -> Result<Self> {
        Ok(Self {
            headers: Vec::new(),
            method: Method::Post,
            uri: Uri::parse(uri)?,
        })
    }

    pub fn put(uri: &'a str) -> Result<Self> {
        Ok(Self {
            headers: Vec::new(),
            method: Method::Put,
            uri: Uri::parse(uri)?,
        })
    }

    pub fn body<T>(self, body: T) -> Request<'a, T> {
        Request {
            header: Header {
                method: self.method,
                uri: self.uri,
                headers: self.headers,
            },
            body,
        }
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
        let req = RequestBuilder::get("https://api.aqsense.no/v1/health").unwrap().body(());

        let buf = req.to_vec().unwrap();

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
        let req = RequestBuilder::post("https://google.com/").unwrap().body(body);

        let buf = req.to_vec().unwrap();

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
        assert_eq!(ct.value, crate::mime::TEXT_PLAIN_UTF_8.as_ref());

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        assert_eq!(&buf[body_status.unwrap()..], body.as_bytes());
    }

    #[test]
    fn build_byte_body() {
        let body = b"hei";
        let req = RequestBuilder::post("https://google.com/").unwrap().body(body.as_slice());

        let buf = req.to_vec().unwrap();

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
        assert_eq!(ct.value, crate::mime::APPLICATION_OCTET_STREAM.inner.as_ref());

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        assert_eq!(&buf[body_status.unwrap()..], body);
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn build_json_body() {
        let body = "hei";
        let req = RequestBuilder::post("https://google.com/").unwrap().body(body);

        let buf = req.to_json_vec().unwrap();

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
        assert_eq!(ct.value, crate::mime::APPLICATION_JSON.as_ref());

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

        fn content_type<'a>(&'a self) -> Option<HeaderValue<'a>> {
            Some("application/test".into())
        }

        fn content_length(&self) -> Option<usize> {
            Some(8)
        }
    }

    #[test]
    fn build_custom() {
        let body = TestStruct { a: 1, b: 2 };
        let req = RequestBuilder::post("https://google.com/").unwrap().body(&body);

        let buf = req.to_vec().unwrap();


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
        let req = RequestBuilder::post("https://google.com/").unwrap().body(&body);


        let buf = req.to_json_vec().unwrap();

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
        assert_eq!(ct.value, crate::mime::APPLICATION_JSON.as_ref());

        // check validity of request
        assert!(body_status.is_complete());

        // check body
        let new_body: TestStruct = serde_json::from_slice(&buf[body_status.unwrap()..]).unwrap();

        assert_eq!(new_body, body);
    }
}
