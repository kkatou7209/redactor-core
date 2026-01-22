/// PDF Hexadecimal String representation (i.e `<4A6F686E>`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexadecimalString {
    /// The hexadecimal characters comprising the string.
    chars: Vec<HexadecimalChar>,
}

impl HexadecimalString {
    
    /// Creates a new `HexadecimalString` from a vector of `HexadecimalChar`.
    pub fn new(chars: Vec<HexadecimalChar>) -> Self {
        
        Self {
            chars,
        }
    }

    /// Returns the characters of the Hexadecimal String.
    pub fn chars(&self) -> &[HexadecimalChar] {
        
        &self.chars
    }
}

/// PDF Hexadecimal string character representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexadecimalChar(u8);

impl HexadecimalChar {
    
    /// Creates a new `HexadecimalChar` character sequence from a byte vector.
    pub fn new(char: u8) -> Self {

        Self(char)
    }
}