use crate::util::check::is_valid_byte_marker_value;

/// PDF Byte Marker representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteMarker {
    value: Vec<u8>,
    bytes: Vec<u8>,
}

impl ByteMarker {

    /// Creates a new `ByteMarker` from the given byte vector.
    pub fn new(value: Vec<u8>) -> Self {

        if !is_valid_byte_marker_value(&value) {
            panic!("Value of byte marker contains invalid bytes. A byte marker must consist of bytes in the range 0x00 to 0xFF.");
        }

        let bytes = format!("%{}", String::from_utf8_lossy(&value)).into_bytes();

        Self { value, bytes }
    }

    /// Returns the value of the byte marker.
    pub fn value(&self) -> &[u8] {
        &self.value
    }

    /// Returns the byte representation of the byte marker.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}