use std::ops::Deref;

/// PDF Whitespace characters representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Whitespace {
    /// Null character.
    /// 
    /// code: `0x00`
    Null,
    /// Space character.
    /// 
    /// code: `0x20`
    Space,
    /// Tab character.
    /// 
    /// code: `0x09`
    Tab,
    /// LF(Line Feed) character.
    /// 
    /// code: `0x0A`
    LineFeed,
    /// FF(Form Feed) character.
    /// 
    /// code: `0x0C`
    FormFeed,
    /// CR(Carriage Return) character.
    /// 
    /// code: `0x0D`
    CarriageReturn,
}

impl Deref for Whitespace {
    
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            Whitespace::Null => &b'\x00',
            Whitespace::Space => &b' ',
            Whitespace::Tab => &b'\t',
            Whitespace::LineFeed => &b'\n',
            Whitespace::FormFeed => &b'\x0C',
            Whitespace::CarriageReturn => &b'\r',
        }
    }
}