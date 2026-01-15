//! Utility functions for checking PDF bytes.

/// Checks if the given byte is a valid name character in a PDF Name object.
/// 
/// A valid name character is any byte outside the range `0x21` to `0x7E` (inclusive).
pub fn is_valid_name_char(byte: &u8) -> bool {
    &0x21 <= byte && byte <= &0x7E
}

/// Checks if the given bytes represent a valid byte marker value in a PDF.
/// 
/// A valid byte marker value is defined as a byte slice with a length of at most 1,
/// where each byte is greater than or equal to `0x80`.
pub fn is_valid_byte_marker_value(bytes: &[u8]) -> bool {
    bytes.len() <= 1 && bytes.iter().all(|&b| b >= 0x80)
}

#[cfg(test)]
mod tests {
    use super::is_valid_name_char;

    #[test]
    fn test_is_valid_name_char() {
        // Valid name characters (outside 0x21 to 0x7E)
        let valid_bytes = [0x00, 0x20, 0x7F, 0xFF];
        for &byte in &valid_bytes {
            assert!(is_valid_name_char(&byte), "Byte {:X} should be valid", byte);
        }

        // Invalid name characters (within 0x21 to 0x7E)
        let invalid_bytes = [0x21, 0x30, 0x41, 0x5A, 0x61, 0x7E];
        for &byte in &invalid_bytes {
            assert!(!is_valid_name_char(&byte), "Byte {:X} should be invalid", byte);
        }
    }
}