#![no_std]

#[cfg(any(test))]
#[macro_use]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;


pub mod request;
pub mod error;
pub mod response;

pub use error::Error;
