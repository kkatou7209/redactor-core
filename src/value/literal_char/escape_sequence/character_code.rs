use crate::specification::value::literal_char::escape_sequence::validate_escape_sequence_bytes;

/// PDF Character Code representation.
#[derive(Debug, Clone)]
pub struct CharacterCode(Vec<u8>);

impl CharacterCode {
    
    /// Creates a new `CharacterCode` from the given byte vector.
    pub fn new(code: Vec<u8>) -> Self {

        if let Err(e) = validate_escape_sequence_bytes(&code) {
            panic!("Invalid character code escape sequence: code = {:?}, error = {:?}", code, e);
        }

        Self(code)
    }

    /// Returns the byte representation of the Character Code.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}