use std::ops::Index;

/// A byte source backed by an in-memory vector of bytes.
pub struct MemoryByteSource {
    data: Vec<u8>,
}

impl Index<usize> for MemoryByteSource {

    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.data[index]
    }
}

impl super::ByteSource for MemoryByteSource {
    fn len(&self) -> usize {
        self.data.len()
    }
}