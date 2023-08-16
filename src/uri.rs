use crate::{Error, Result};

use alloc::borrow::Cow;
use core::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri<'a> {
    pub inner: Cow<'a, str>,
    scheme: Range<usize>,
    authority: Range<usize>,
    path_and_query: Range<usize>,
}

impl<'a> Uri<'a> {
    pub fn parse<S: Into<Cow<'a, str>>>(uri: S) -> Result<Self> {
        let mut start_idx = 0;

        let s = uri.into();

        let scheme = match s.find("://") {
            Some(idx) => {
                let scheme = start_idx..idx;
                start_idx = idx + 3;
                scheme
            }
            None => return Err(Error::InvalidUri)
        };

        let authority = match s[start_idx..].find('/') {
            Some(idx) => {
                let authority = start_idx..start_idx + idx;
                start_idx = start_idx + idx;
                authority
            }
            None => {
                return Err(Error::InvalidUri);
            }
        };

        let path_and_query = start_idx..s.len();

        Ok(Self {
            inner: s,
            scheme,
            authority,
            path_and_query,
        })
    }

    pub fn into_owned(self) -> Uri<'static> {
        Uri {
            inner: Cow::Owned(self.inner.into_owned()),
            scheme: self.scheme.clone(),
            authority: self.authority.clone(),
            path_and_query: self.path_and_query.clone(),
        }
    }

    pub fn into_borrowed<'c: 'a>(&'c self) -> Uri<'a> {
        Uri {
            inner: Cow::Borrowed(self.inner.as_ref()),
            scheme: self.scheme.clone(),
            authority: self.authority.clone(),
            path_and_query: self.path_and_query.clone(),
        }
    }

    pub fn scheme(&self) -> &str {
        &self.inner[self.scheme.clone()]
    }

    pub fn authority(&self) -> &str {
        &self.inner[self.authority.clone()]
    }

    pub fn path_and_query(&self) -> &str {
        &self.inner[self.path_and_query.clone()]
    }
}


impl<'a> TryFrom<Cow<'a, str>> for Uri<'a> {
    type Error = Error;

    fn try_from(s: Cow<'a, str>) -> Result<Self> {
        Self::parse(s)
    }
}

impl<'a> TryFrom<&'a str> for Uri<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        Self::parse(s)
    }
}

impl TryFrom<alloc::string::String> for Uri<'static> {
    type Error = Error;

    fn try_from(s: alloc::string::String) -> Result<Self> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static URIS: &[&str] = &[
        "https://www.google.com/",
        "ws://test.com/",
        "http://test.com/asdf/1234",
        "http://test.com/asdf/1234?asdf=1234",
        "http://test.com/asdf/1234?asdf=1234&asdf=1234",
    ];

    #[test]
    fn test_parse() {
        for &uri in URIS {
            let uri1 = Uri::parse(uri).unwrap();
            let uri2 = http::Uri::from_static(uri);
            assert_eq!(uri2.scheme_str().unwrap(), uri1.scheme());
            assert_eq!(uri2.authority().unwrap(), uri1.authority());
            assert_eq!(uri2.path_and_query().unwrap(), uri1.path_and_query());
        }
    }

    #[test]
    fn test_into_owned() {
        let uri = Uri::parse("https://www.google.com/").unwrap();

        let uri2 = uri.clone().into_owned();
        assert_eq!(uri2, uri);

        let uri3 = uri2.into_borrowed();

        assert_eq!(uri3, uri);
    }
}

