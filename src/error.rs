#[cfg(feature = "alloc")]
use crate::alloc::string::ToString;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "serde_json")]
    SerdeError(serde_json::Error),
    BufferTooSmall(usize, usize),
    FmtError(core::fmt::Error),
    Other(&'static str),
}

#[cfg(feature = "defmt")]
impl defmt::Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            #[cfg(feature = "serde_json")]
            Error::SerdeError(e) => {
                #[cfg(not(feature = "alloc"))]
                defmt::write!(fmt, "SerdeError()");

                #[cfg(feature = "alloc")]
                defmt::write!(fmt, "SerdeError({})", e.to_string());
            }
            Error::BufferTooSmall(s1, s2) => {
                defmt::write!(fmt, "BufferTooSmall({}, {})", s1, s2);
            }
            Error::FmtError(e) => {
                #[cfg(feature = "alloc")]
                defmt::write!(fmt, "FmtError({})", e.to_string());

                #[cfg(not(feature = "alloc"))]
                defmt::write!(fmt, "FmtError()");
            }
            Error::Other(s) => {
                defmt::write!(fmt, "Other({})", s);
            }
        }
        // Format as hexadecimal.
    }
}

impl From<core::fmt::Error> for Error {
    fn from(e: core::fmt::Error) -> Self {
        Self::FmtError(e)
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}

impl From<Error> for core::fmt::Error {
    fn from(_: Error) -> Self {
        core::fmt::Error
    }
}
