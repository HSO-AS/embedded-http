use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use super::*;

#[cfg(feature = "alloc")]
pub struct VecWriter {
    inner: Vec<u8>,
}

impl Write for VecWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.inner.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

impl Writer for VecWriter {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.inner.extend_from_slice(bytes);
        Ok(())
    }

    fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }
}

impl Deref for VecWriter {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for VecWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

impl From<Vec<u8>> for VecWriter {
    fn from(v: Vec<u8>) -> Self {
        Self {
            inner: v
        }
    }
}