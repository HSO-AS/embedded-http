
use super::*;

pub struct SliceWriter<const D: usize = 512> {
    buffer: [u8; D],
    idx: usize,
}

impl Writer for SliceWriter {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.extend(bytes)?;
        Ok(())
    }

    fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.idx]
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer[..self.idx]
    }
}

impl<const D: usize> SliceWriter<D> {
    pub fn new(slice: [u8; D]) -> Self {
        Self { buffer: slice, idx: 0 }
    }

    pub fn extend(&mut self, buffer: &[u8]) -> Result<(), Error> {
        if self.buffer.len() - self.idx < buffer.len() {
            Err(Error::BufferTooSmall(self.buffer.len() - self.idx, buffer.len()))
        } else {
            self.buffer[self.idx..self.idx + buffer.len()].iter_mut().zip(buffer).for_each(|(s, v)| *s = *v);
            self.idx += buffer.len();
            Ok(())
        }
    }
}


impl Write for SliceWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.extend(s.as_bytes())?;
        Ok(())
    }
}