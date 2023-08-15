use crate::{Error, Result};

use alloc::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri<'a> {
    pub scheme: Cow<'a, str>,
    pub authority: Cow<'a, str>,
    pub path_and_query: Cow<'a, str>,
}

impl<'a> Uri<'a> {
    pub fn parse(s: &'a str) -> Result<Self> {
        let mut start_idx = 0;

        let scheme = match s.find("://") {
            Some(idx) => {
                let scheme = Cow::Borrowed(&s[start_idx..idx]);
                start_idx = idx + 3;
                scheme
            }
            None => return Err(Error::InvalidUri)
        };

        let authority = match s[start_idx..].find('/') {
            Some(idx) => {
                let authority = Cow::Borrowed(&s[start_idx..start_idx + idx]);
                start_idx = start_idx + idx;
                authority
            }
            None => {
                return Err(Error::InvalidUri);
            }
        };

        let path_and_query = Cow::Borrowed(&s[start_idx..]);

        Ok(Self {
            scheme,
            authority,
            path_and_query,
        })
    }

    pub fn into_owned(self) -> Uri<'static> {
        Uri {
            scheme: Cow::Owned(self.scheme.into_owned()),
            authority: Cow::Owned(self.authority.into_owned()),
            path_and_query: Cow::Owned(self.path_and_query.into_owned()),
        }
    }

    pub fn into_borrowed<'b: 'a>(&'b self) -> Uri<'a> {
        Uri {
            scheme: Cow::Borrowed(self.scheme.as_ref()),
            authority: Cow::Borrowed(self.authority.as_ref()),
            path_and_query: Cow::Borrowed(self.path_and_query.as_ref()),
        }
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
        for uri in URIS {
            let uri1 = Uri::parse(uri).unwrap();
            let uri2 = http::Uri::from_static(uri);
            assert_eq!(uri2.scheme_str().unwrap(), uri1.scheme.as_ref());
            assert_eq!(uri2.authority().unwrap(), uri1.authority.as_ref());
            assert_eq!(uri2.path_and_query().unwrap(), uri1.path_and_query.as_ref());
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

