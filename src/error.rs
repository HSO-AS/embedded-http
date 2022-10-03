#[cfg(all(feature = "defmt", feature = "alloc"))]
use defmt::warn;

use embedded_io::blocking::WriteFmtError;
use embedded_io::ErrorKind;

#[cfg(all(feature = "alloc", feature = "serde_json"))]
use crate::alloc::string::ToString;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "serde_json")]
    SerdeError(serde_json::Error),
    BufferTooSmall(usize, usize),
    FmtError,
    Other(&'static str),
    IoError(embedded_io::ErrorKind),
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
            Error::FmtError => {
                defmt::write!(fmt, "FmtError");
            }
            Error::Other(s) => {
                defmt::write!(fmt, "Other({})", s);
            }
            Error::IoError(k) => {
                defmt::write!(fmt, "IoError({})", k);
            }
        }
        // Format as hexadecimal.
    }
}

impl From<core::fmt::Error> for Error {
    fn from(e: core::fmt::Error) -> Self {
        #[cfg(all(feature = "defmt", feature = "alloc"))]
        warn!("FmtError({})", e);

        Self::FmtError
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


impl From<&dyn embedded_io::Error> for Error {
    fn from(e: &dyn embedded_io::Error) -> Self {
        Self::from(e.kind())
    }
}

impl<E> From<WriteFmtError<E>> for Error {
    fn from(e: WriteFmtError<E>) -> Self {
        match e {
            WriteFmtError::FmtError => { Error::FmtError }
            WriteFmtError::Other(_) => {
                Error::IoError(ErrorKind::Other)
            }
        }
    }
}


impl From<ErrorKind> for Error {
    fn from(ek: ErrorKind) -> Self {
        Self::IoError(ek)
    }
}