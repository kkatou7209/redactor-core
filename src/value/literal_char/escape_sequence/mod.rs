mod character_code;

use character_code::CharacterCode;

/// PDF escape sequence representation.
#[derive(Debug, Clone)]
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
    CharacterCode(CharacterCode),
    /// Backslash only escape sequence (`\`).
    Empty,
    /// End of line escape sequence (`\<LF>`).
    /// 
    /// A Line Feed (`\n`) or Carriage Return (`\r`) immediately
    /// following a backslash (`\`) is treated as a just Line Feed.
    EndOfLine,
}

impl EscapeSequence {
    
    /// Returns the byte representation of the escape sequence.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            EscapeSequence::LineFeed => br"\n",
            EscapeSequence::CarriageReturn => br"\r",
            EscapeSequence::Tab => br"\t",
            EscapeSequence::Backspace => br"\\",
            EscapeSequence::FormFeed => br"\f",
            EscapeSequence::LeftParenthesis => br"\(",
            EscapeSequence::RightParenthesis => br"\)",
            EscapeSequence::Backslash => br"\\",
            EscapeSequence::CharacterCode(code) => code.as_bytes(),
            EscapeSequence::Empty => br"\",
            EscapeSequence::EndOfLine => b"\n",
        }
    }
}