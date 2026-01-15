use crate::structure::eof::EOF;
use crate::structure::header::Header;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralComment {
    EOF(EOF),
    Header(Header),
}