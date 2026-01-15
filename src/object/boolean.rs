use std::ops::Deref;

/// PDF Boolean object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean(bool);

impl Boolean {
    
    /// Creates a new Boolean from a Rust bool.
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the Rust bool.
    pub fn as_bool(&self) -> bool {
        self.0
    }

    /// Returns the byte representation of the Boolean.
    pub fn as_bytes(&self) -> &[u8] {
        if self.0 {
            b"true"
        } else {
            b"false"
        }
    }
}

impl Deref for Boolean {
    
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}