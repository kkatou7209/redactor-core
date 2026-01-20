/// Checks if the given byte slice represents a valid escape sequence
/// in a PDF literal string.
pub fn validate_escape_sequence_bytes(bytes: &[u8]) -> Result<(), String> {

    if !matches!(bytes.len(), 1..=4) {
        return Err(format!("Invalid escape sequence length: {}", bytes.len()));
    }

    if matches!(bytes, b"\\" | b"\\n" | b"\\r" | b"\\t" | b"\\b" | b"\\f" | b"\\(" | b"\\)" | b"\\\\" | b"\\\r\n") {
        return Ok(());
    }

    if bytes[0] != b'\\' {
        return Err(format!("Escape sequence must start with backslash: {:?}", bytes));
    }

    let may_be_digits = &bytes[1..];

    if !may_be_digits.iter().all(|&b| b.is_ascii_digit()) {
        return Err(format!("Escape sequence contains non-digit characters: {:?}", bytes));
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::specification::value::literal_char::escape_sequence::validate_escape_sequence_bytes;

    #[test]
    fn should_validate_escape_sequences() {

        let valid_sequences: &[&[u8]] = &[
            b"\\n",
            b"\\r",
            b"\\t",
            b"\\b",
            b"\\f",
            b"\\(",
            b"\\)",
            b"\\\\",
            b"\\\r\n",
            b"\\123",
            b"\\0",
            b"\\12",
        ];

        for (index, seq) in valid_sequences.iter().enumerate() {

            assert!(validate_escape_sequence_bytes(seq).is_ok(), "Expected valid: {:?} at index {}", seq, index);
        }
    }

    #[test]
    fn should_invalidate_escape_sequences() {

        let invalid_sequences: &[&[u8]] = &[
            b"",
            b"\\x",
            b"\\a",
            b"\\1234",
            b"\\1a",
            b"abc",
            b"\\ \r\n",
        ];

        for (index, seq) in invalid_sequences.iter().enumerate() {
            assert!(validate_escape_sequence_bytes(seq).is_err(), "Expected invalid: {:?} at index {}", seq, index);
        }
    }
}