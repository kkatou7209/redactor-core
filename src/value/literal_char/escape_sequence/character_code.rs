use crate::specification::value::literal_char::escape_sequence::{validate_escape_sequence_bytes, validate_escaped_char_code};

/// PDF escaped character code representation.
#[derive(Debug, Clone)]
pub struct CharacterCode(Vec<u8>);

impl CharacterCode {
    
    /// Creates a new `CharacterCode` from the given byte vector.
    pub fn new(code: Vec<u8>) -> Self {

        if let Err(e) = validate_escaped_char_code(&code) {
            panic!("Invalid character code escape sequence: code = {:?}, error = {:?}", code, e);
        }

        Self(code)
    }

    /// Returns the byte representation of the Character Code.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Converts the character code to a u32 value.
    pub fn as_u32(&self) -> u32 {

        if self.0.len() < 2 {
            panic!("Character code is too short: {:?}", self.0);
        }

        let mut value: u32 = 0;
        
        for &byte in &self.0[1..] {

            if !byte.is_ascii_digit() {
                panic!("Character code contains non-digit byte: {:?}", self.0);
            }

            value = value * 10 + (byte - b'0') as u32;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::CharacterCode;

    #[test]
    fn should_create_valid_character_code() {
        let char_code = CharacterCode::new(b"\\123".to_vec());
        assert_eq!(char_code.as_bytes(), b"\\123");
        assert_eq!(char_code.as_u32(), 123);
    }
}