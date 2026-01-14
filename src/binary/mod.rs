//! Module for binary data handling.
mod memory_byte_source;

use std::ops::Index;

/// A trait representing a source of bytes that can be indexed by `usize`.
/// 
/// The data source allows read-only access to its bytes.
pub(crate) trait ByteSource: Index<usize, Output = u8> {
    /// Returns the length of the byte source.
    fn len(&self) -> usize;
}