use crate::header::{ HeaderValue};

pub(crate) const APPLICATION_JSON: HeaderValue = HeaderValue::from_static(b"application/json");
pub(crate) const APPLICATION_OCTET_STREAM: HeaderValue = HeaderValue::from_static(b"application/octet-stream");
pub(crate) const TEXT_PLAIN_UTF_8: HeaderValue = HeaderValue::from_static(b"text/plain; charset=utf-8");