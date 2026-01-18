use crate::value::LiteralCharacter;

/// PDF Literal String representation.
#[derive(Debug, Clone)]
pub struct LiteralString(Vec<LiteralCharacter>);

impl LiteralString {
    
    /// Creates a new `LiteralString` from the given vector of `LiteralCharacter`.
    pub fn new(characters: Vec<LiteralCharacter>) -> Self {
        Self(characters)
    }

    /// Returns the characters of the Literal String.
    pub fn characters(&self) -> &[LiteralCharacter] {
        &self.0
    }

    /// Returns the byte representation of the Literal String.
    pub fn as_bytes(&self) -> Vec<u8> {

        let mut bytes = vec![b'('];
        
        for (index, character) in self.0.iter().enumerate() {

            if index != 0 {
                bytes.push(b' ');
            }

            bytes.extend_from_slice(character.as_bytes());
        }

        bytes.push(b')');

        bytes
    }
}