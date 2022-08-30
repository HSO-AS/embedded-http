pub mod slice_writer;

#[cfg(feature = "alloc")]
pub mod vec_writer;

use crate::Error;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::fmt::Write;


pub trait Writer: Write {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error>;

    fn as_slice(&self) -> &[u8];

    fn as_mut_slice(&mut self) -> &mut [u8];
}


/*
#[cfg(feature = "alloc")]
impl VecWriter {
    pub fn new(vec: Vec<u8>) -> Self {
        Self { inner: vec }
    }
}

 */
/*
#[cfg(feature="alloc")]
impl core::ops::Deref for VecWriter {
    type Target = Vec<u8>;

    #[inline]
    fn deref(&self) -> &Vec<u8> {
        &self.inner
    }
}

#[cfg(feature="alloc")]
impl core::ops::DerefMut for VecWriter {
    #[inline]
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}
 */




/*
#[cfg(feature = "alloc")]
impl<W: std::io::Write + core::fmt::Write> Writer for W {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        todo!()
    }

    fn as_slice(&self) -> &[u8] {
        todo!()
    }
}
*/



/*
impl From<Vec<u8>> for VecWriter {
    fn from(v: Vec<u8>) -> Self {
        Self::new(v)
    }
}

#[cfg(feature="alloc")]
impl std::fmt::Write for VecWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(self.extend_from_slice(s.as_bytes()))
    }
}

 */

/*
#[cfg(feature = "alloc")]
impl Writer for VecWriter {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        Ok(self.extend_from_slice(bytes))
    }

    fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    fn as_mut_writer(&mut self) -> &mut [u8]{
        self.inner.as_mut_slice()
    }
}
 */

/*
#[cfg(feature = "alloc")]
impl std::io::Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
*/

/*
impl<W: Writer> std::io::Write for W {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_bytes(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

 */