use std::ops::Deref;

/// PDF character representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Char {
    /// The bytes of the character.
    bytes: Vec<u8>,
}

impl Char {
    /// Creates a new `Char` from a single byte.
    pub fn new(bytes: &[u8]) -> Self {
        Self { bytes: bytes.to_vec() }
    }

    /// Returns the byte representation of the character.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Deref for Char {
    
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}