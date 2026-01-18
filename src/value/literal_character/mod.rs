use crate::value::literal_character::ascii::Ascii;
use crate::value::literal_character::escape_sequence::EscapeSequence;

mod ascii;
mod escape_sequence;

/// PDF Literal string character representation.
#[derive(Debug, Clone)]
pub enum LiteralCharacter {
    /// PDF ASCII character representation.
    Ascii(Ascii),
    /// PDF Escape Sequence character representation.
    EscapeSequence(EscapeSequence),
}

impl LiteralCharacter {
    
    /// Returns the byte representation of the Literal Character.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            LiteralCharacter::Ascii(ascii) => ascii.as_bytes(),
            LiteralCharacter::EscapeSequence(escape_sequence) => escape_sequence.as_bytes(),
        }
    }
}