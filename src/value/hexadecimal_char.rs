use crate::specification::value::hexadecimal_char::validate_hexadecimal_char;

/// PDF Hexadecimal string character representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexadecimalChar(Vec<u8>);

impl HexadecimalChar {
    
    /// Creates a new `HexadecimalChar` character sequence from a byte vector.
    pub fn new(bytes: &[u8]) -> Self {

        if let Err(e) = validate_hexadecimal_char(bytes) {
            panic!("The provided byte is not a hexadecimal character: {}", e);
        }
        
        Self(bytes.to_vec())
    }

    /// Returns the byte representation of the Hexadecimal character sequence.
    pub fn as_bytes(&self) -> &[u8] {

        &self.0
    }
}