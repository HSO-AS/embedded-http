//! Sample of common used HTTP header keys.
//!

use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderKey<'a> {
    pub inner: Cow<'a, str>,
}

impl Display for HeaderKey<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderValue<'a> {
    pub inner: Cow<'a, [u8]>,
}

impl AsRef<[u8]> for HeaderValue<'_> {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl<'a> HeaderKey<'a> {
    pub const fn from_static(s: &'static str) -> HeaderKey<'static> {
        HeaderKey {
            inner: Cow::Borrowed(s),
        }
    }

    pub fn into_owned(self) -> HeaderKey<'static> {
        HeaderKey {
            inner: Cow::Owned(self.inner.into_owned()),
        }
    }

    pub fn into_borrowed<'b: 'a>(&'b self) -> HeaderKey<'a> {
        HeaderKey {
            inner: Cow::Borrowed(self.inner.as_ref()),
        }
    }
}

impl<'a> HeaderValue<'a> {
    pub const fn from_static(s: &'static [u8]) -> HeaderValue<'static> {
        HeaderValue {
            inner: Cow::Borrowed(s),
        }
    }

    pub fn into_owned(self) -> HeaderValue<'static> {
        HeaderValue {
            inner: Cow::Owned(self.inner.into_owned()),
        }
    }

    pub fn into_borrowed<'b: 'a>(&'b self) -> HeaderValue<'a> {
        HeaderValue {
            inner: Cow::Borrowed(self.inner.as_ref()),
        }
    }
}

impl<'a> From<&'a str> for HeaderKey<'a> {
    fn from(s: &'a str) -> Self {
        HeaderKey {
            inner: Cow::Borrowed(s),
        }
    }
}

impl From<String> for HeaderValue<'static> {
    fn from(s: String) -> Self {
        Self {
            inner: Cow::Owned(s.into_bytes()),
        }
    }
}

impl From<String> for HeaderKey<'static> {
    fn from(s: String) -> Self {
        Self {
            inner: Cow::Owned(s),
        }
    }
}

impl<'a> From<&'a str> for HeaderValue<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            inner: Cow::Borrowed(s.as_bytes()),
        }
    }
}

impl<'a> From<&'a [u8]> for HeaderValue<'a> {
    fn from(s: &'a [u8]) -> Self {
        Self {
            inner: Cow::Borrowed(s),
        }
    }
}

pub static ACCEPT: HeaderKey<'static> = HeaderKey::from_static("accept");

pub static ACCEPT_CHARSET: HeaderKey<'static> = HeaderKey::from_static("accept-charset");

pub static ACCEPT_ENCODING: HeaderKey<'static> = HeaderKey::from_static("accept-encoding");

pub static ACCEPT_LANGUAGE: HeaderKey<'static> = HeaderKey::from_static("accept-language");

pub static ACCEPT_RANGES: HeaderKey<'static> = HeaderKey::from_static("accept-ranges");

pub static ACCESS_CONTROL_ALLOW_CREDENTIALS: HeaderKey<'static> =
    HeaderKey::from_static("access-control-allow-credentials");

pub static ACCESS_CONTROL_ALLOW_HEADERS: HeaderKey<'static> =
    HeaderKey::from_static("access-control-allow-headers");

pub static ACCESS_CONTROL_ALLOW_METHODS: HeaderKey<'static> =
    HeaderKey::from_static("access-control-allow-methods");

pub static ACCESS_CONTROL_ALLOW_ORIGIN: HeaderKey<'static> =
    HeaderKey::from_static("access-control-allow-origin");

pub static ACCESS_CONTROL_EXPOSE_HEADERS: HeaderKey<'static> =
    HeaderKey::from_static("access-control-expose-headers");

pub static ACCESS_CONTROL_MAX_AGE: HeaderKey<'static> =
    HeaderKey::from_static("access-control-max-age");

pub static ACCESS_CONTROL_REQUEST_HEADERS: HeaderKey<'static> =
    HeaderKey::from_static("access-control-request-headers");

pub static ACCESS_CONTROL_REQUEST_METHOD: HeaderKey<'static> =
    HeaderKey::from_static("access-control-request-method");

pub static AGE: HeaderKey<'static> = HeaderKey::from_static("age");

pub static ALLOW: HeaderKey<'static> = HeaderKey::from_static("allow");

pub static ALT_SVC: HeaderKey<'static> = HeaderKey::from_static("alt-svc");

pub static AUTHORIZATION: HeaderKey<'static> = HeaderKey::from_static("authorization");

pub static CACHE_CONTROL: HeaderKey<'static> = HeaderKey::from_static("cache-control");

pub static CACHE_STATUS: HeaderKey<'static> = HeaderKey::from_static("cache-status");

pub static CDN_CACHE_CONTROL: HeaderKey<'static> = HeaderKey::from_static("cdn-cache-control");

pub static CONNECTION: HeaderKey<'static> = HeaderKey::from_static("connection");

pub static CONTENT_DISPOSITION: HeaderKey<'static> = HeaderKey::from_static("content-disposition");

pub static CONTENT_ENCODING: HeaderKey<'static> = HeaderKey::from_static("content-encoding");

pub static CONTENT_LANGUAGE: HeaderKey<'static> = HeaderKey::from_static("content-language");

pub static CONTENT_LENGTH: HeaderKey<'static> = HeaderKey::from_static("content-length");

pub static CONTENT_LOCATION: HeaderKey<'static> = HeaderKey::from_static("content-location");

pub static CONTENT_RANGE: HeaderKey<'static> = HeaderKey::from_static("content-range");

pub static CONTENT_SECURITY_POLICY: HeaderKey<'static> =
    HeaderKey::from_static("content-security-policy");

pub static CONTENT_SECURITY_POLICY_REPORT_ONLY: HeaderKey<'static> =
    HeaderKey::from_static("content-security-policy-report-only");

pub static CONTENT_TYPE: HeaderKey<'static> = HeaderKey::from_static("content-type");

pub static COOKIE: HeaderKey<'static> = HeaderKey::from_static("cookie");

pub static DNT: HeaderKey<'static> = HeaderKey::from_static("dnt");

pub static DATE: HeaderKey<'static> = HeaderKey::from_static("date");

pub static ETAG: HeaderKey<'static> = HeaderKey::from_static("etag");

pub static EXPECT: HeaderKey<'static> = HeaderKey::from_static("expect");

pub static EXPIRES: HeaderKey<'static> = HeaderKey::from_static("expires");

pub static FORWARDED: HeaderKey<'static> = HeaderKey::from_static("forwarded");

pub static FROM: HeaderKey<'static> = HeaderKey::from_static("from");

pub static HOST: HeaderKey<'static> = HeaderKey::from_static("host");

pub static IF_MATCH: HeaderKey<'static> = HeaderKey::from_static("if-match");

pub static IF_MODIFIED_SINCE: HeaderKey<'static> = HeaderKey::from_static("if-modified-since");

pub static IF_NONE_MATCH: HeaderKey<'static> = HeaderKey::from_static("if-none-match");

pub static IF_RANGE: HeaderKey<'static> = HeaderKey::from_static("if-range");

pub static IF_UNMODIFIED_SINCE: HeaderKey<'static> = HeaderKey::from_static("if-unmodified-since");

pub static LAST_MODIFIED: HeaderKey<'static> = HeaderKey::from_static("last-modified");

pub static LINK: HeaderKey<'static> = HeaderKey::from_static("link");

pub static LOCATION: HeaderKey<'static> = HeaderKey::from_static("location");

pub static MAX_FORWARDS: HeaderKey<'static> = HeaderKey::from_static("max-forwards");

pub static ORIGIN: HeaderKey<'static> = HeaderKey::from_static("origin");

pub static PRAGMA: HeaderKey<'static> = HeaderKey::from_static("pragma");

pub static PROXY_AUTHENTICATE: HeaderKey<'static> = HeaderKey::from_static("proxy-authenticate");

pub static PROXY_AUTHORIZATION: HeaderKey<'static> = HeaderKey::from_static("proxy-authorization");

pub static PUBLIC_KEY_PINS: HeaderKey<'static> = HeaderKey::from_static("public-key-pins");

pub static PUBLIC_KEY_PINS_REPORT_ONLY: HeaderKey<'static> =
    HeaderKey::from_static("public-key-pins-report-only");

pub static RANGE: HeaderKey<'static> = HeaderKey::from_static("range");

pub static REFERER: HeaderKey<'static> = HeaderKey::from_static("referer");

pub static REFERRER_POLICY: HeaderKey<'static> = HeaderKey::from_static("referrer-policy");

pub static REFRESH: HeaderKey<'static> = HeaderKey::from_static("refresh");

pub static RETRY_AFTER: HeaderKey<'static> = HeaderKey::from_static("retry-after");

pub static SEC_WEBSOCKET_ACCEPT: HeaderKey<'static> =
    HeaderKey::from_static("sec-websocket-accept");

pub static SEC_WEBSOCKET_EXTENSIONS: HeaderKey<'static> =
    HeaderKey::from_static("sec-websocket-extensions");

pub static SEC_WEBSOCKET_KEY: HeaderKey<'static> = HeaderKey::from_static("sec-websocket-key");

pub static SEC_WEBSOCKET_PROTOCOL: HeaderKey<'static> =
    HeaderKey::from_static("sec-websocket-protocol");

pub static SEC_WEBSOCKET_VERSION: HeaderKey<'static> =
    HeaderKey::from_static("sec-websocket-version");

pub static SERVER: HeaderKey<'static> = HeaderKey::from_static("server");

pub static SET_COOKIE: HeaderKey<'static> = HeaderKey::from_static("set-cookie");

pub static STRICT_TRANSPORT_SECURITY: HeaderKey<'static> =
    HeaderKey::from_static("strict-transport-security");

pub static TE: HeaderKey<'static> = HeaderKey::from_static("te");

pub static TRAILER: HeaderKey<'static> = HeaderKey::from_static("trailer");

pub static TRANSFER_ENCODING: HeaderKey<'static> = HeaderKey::from_static("transfer-encoding");

pub static USER_AGENT: HeaderKey<'static> = HeaderKey::from_static("user-agent");

pub static UPGRADE: HeaderKey<'static> = HeaderKey::from_static("upgrade");

pub static UPGRADE_INSECURE_REQUESTS: HeaderKey<'static> =
    HeaderKey::from_static("upgrade-insecure-requests");

pub static VARY: HeaderKey<'static> = HeaderKey::from_static("vary");

pub static VIA: HeaderKey<'static> = HeaderKey::from_static("via");

pub static WARNING: HeaderKey<'static> = HeaderKey::from_static("warning");

pub static WWW_AUTHENTICATE: HeaderKey<'static> = HeaderKey::from_static("www-authenticate");

pub static X_CONTENT_TYPE_OPTIONS: HeaderKey<'static> =
    HeaderKey::from_static("x-content-type-options");

pub static X_DNS_PREFETCH_CONTROL: HeaderKey<'static> =
    HeaderKey::from_static("x-dns-prefetch-control");

pub static X_FRAME_OPTIONS: HeaderKey<'static> = HeaderKey::from_static("x-frame-options");

pub static X_XSS_PROTECTION: HeaderKey<'static> = HeaderKey::from_static("x-xss-protection");
