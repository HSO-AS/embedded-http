#![no_std]
#![cfg_attr(feature = "unstable", feature(error_in_core))]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

mod prelude {
    #[cfg(feature = "alloc")]
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
