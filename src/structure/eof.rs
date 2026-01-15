
/// A PDF End-Of-File (EOF) marker representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EOF;

impl EOF {

    /// Creates a new `EOF` marker.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns the byte representation of the EOF marker.
    pub fn as_bytes(&self) -> &[u8] {
        b"%%EOF"
    }
}