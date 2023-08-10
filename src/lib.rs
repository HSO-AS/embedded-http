#![no_std]
#![cfg_attr(feature = "unstable", feature(error_in_core))]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

mod prelude {
    pub use crate::alloc::string::ToString;

    #[cfg(feature = "defmt")]
    pub use defmt::{debug, error, info, warn};

    #[cfg(feature = "serde_json")]
    pub use serde::Serialize;
}

pub mod error;
pub mod request;
pub mod response;

pub use error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;
