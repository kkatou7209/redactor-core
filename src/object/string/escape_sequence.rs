/// PDF escape sequence representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeSequence {
    /// Line feed escape sequence (`\n`).
    LineFeed,
    /// Carriage return escape sequence (`\r`).
    CarriageReturn,
    /// Tab escape sequence (`\t`).
    Tab,
    /// Backspace escape sequence (`\b`).
    Backspace,
    /// Form feed escape sequence (`\f`).
    FormFeed,
    /// Parenthesis left escape sequence (`\(`).
    LeftParenthesis,
    /// Parenthesis right escape sequence (`\)`).
    RightParenthesis,
    /// Backslash escape sequence (`\\`).
    Backslash,
    /// Character code escape sequence (`\ddd`).
    CharacterCode(CharCode),
    /// Backslash only escape sequence (`\`).
    Empty,
    /// End of line escape sequence (`\<LF>`).
    /// 
    /// A Line Feed (`\n`) or Carriage Return (`\r`) immediately
    /// following a backslash (`\`) is treated as a just Line Feed.
    EndOfLine,
}

/// PDF escaped character code representation.
/// 
/// A character code must be between `0` and `255`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharCode(u8);

impl CharCode {
    
    /// Creates a new `CharacterCode` from the given byte vector.
    pub fn new(code: u8) -> Self {

        Self(code)
    }

    /// Returns the character byte.
    pub fn as_byte(&self) -> u8 {
    
        self.0
    }
}