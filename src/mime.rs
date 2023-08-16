use crate::header::{ HeaderValue};

pub static APPLICATION_JSON: HeaderValue = HeaderValue::from_static(b"application/json");
pub static APPLICATION_OCTET_STREAM: HeaderValue = HeaderValue::from_static(b"application/octet-stream");
pub static TEXT_PLAIN_UTF_8: HeaderValue = HeaderValue::from_static(b"text/plain; charset=utf-8");