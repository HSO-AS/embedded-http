use embedded_io::ErrorKind;
use embedded_io::WriteFmtError;

#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Debug)]
pub enum Error {
    FmtError,
    WriteZero,
    #[cfg(feature = "defmt")]
    DefmtFmtError,
    #[cfg(feature = "serde_json")]
    SerdeError(serde_json::Error),
    ErrorKind(ErrorKind),
    Infallible(core::convert::Infallible),
    InvalidUri,
}

#[cfg(feature = "defmt")]
impl defmt::Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Error::DefmtFmtError => {
                defmt::write!(fmt, "DefmtFmtError");
            }
            Error::WriteZero => {
                defmt::write!(fmt, "WriteZero");
            }
            #[cfg(feature = "serde_json")]
            Error::SerdeError(e) => {
                #[cfg(not(feature = "alloc"))]
                defmt::write!(fmt, "SerdeError()");

                #[cfg(feature = "alloc")]
                {
                    use alloc::string::ToString;
                    defmt::write!(fmt, "SerdeError({})", e.to_string());
                }
            }
            Error::FmtError => {
                defmt::write!(fmt, "FmtError");
            }
            Error::ErrorKind(e) => {
                defmt::write!(fmt, "ErrorKind({:?})", e);
            }
            Error::Infallible(e) => {
                defmt::write!(fmt, "Infallible({:?})", e);
            }
            Error::InvalidUri => {
                defmt::write!(fmt, "InvalidUri");
            }
        }
        // Format as hexadecimal.
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E: embedded_io::Error> From<WriteFmtError<E>> for Error {
    fn from(e: WriteFmtError<E>) -> Self {
        match e {
            WriteFmtError::FmtError => Self::FmtError,
            WriteFmtError::Other(e) => Self::ErrorKind(e.kind()),
        }
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(e: core::convert::Infallible) -> Self {
        Self::Infallible(e)
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Self::ErrorKind(e)
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}

impl embedded_io::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::ErrorKind(e) => *e,
            _ => ErrorKind::Other,
        }
    }
}

#[cfg(feature = "unstable")]
mod unstable {
    use super::*;

    impl core::error::Error for Error {}
}
