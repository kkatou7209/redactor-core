use std::ops::Deref;

/// PDF Delimiter characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
    /// Indicates the start of a PDF string.
    /// 
    /// char: `(`
    LeftParen,
    /// Indicates the end of a PDF string.
    /// 
    /// char: `)`
    RightParen,
    /// Indicates the start of a PDF Hexadecimal string.
    /// 
    /// char: `<`
    LeftAngle,
    /// Indicates the end of a PDF hexadecimal string.
    /// 
    /// char: `>`
    RightAngle,
    /// Indicates the start of an `Array`.
    /// 
    /// char: `[`
    LeftSquare,
    /// Indicates the end of an `Array`.
    /// 
    /// char: `]`
    RightSquare,
    /// Indicates no special structure.
    /// 
    /// char: `{`
    LeftCurlyBracket,
    /// Indicates no special structure.
    /// 
    /// char: `}`
    RightCurlyBracket,
    /// Indicates the start of a Name object.
    /// 
    /// char: `/`
    Solidus,
    /// Indicates the start of a comment.
    /// 
    /// char: `%`
    PercentSign,
}

impl Deref for Delimiter {
    
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            Delimiter::LeftParen => &b'(',
            Delimiter::RightParen => &b')',
            Delimiter::LeftAngle => &b'<',
            Delimiter::RightAngle => &b'>',
            Delimiter::LeftSquare => &b'[',
            Delimiter::RightSquare => &b']',
            Delimiter::LeftCurlyBracket => &b'{',
            Delimiter::RightCurlyBracket => &b'}',
            Delimiter::Solidus => &b'/',
            Delimiter::PercentSign => &b'%',
        }
    }
}