/// Validates whether the given byte slice 
/// represents a valid hexadecimal character.
pub fn validate_hexadecimal_char(bytes: &[u8]) -> Result<(), String> {

    if bytes.is_empty() {
        return Err("Hexadecimal character sequence cannot be empty.".to_string());
    }

    if bytes.len() != 2 {
        return Err(format!("Hexadecimal character sequence must be exactly 2 characters long, got length: {}", bytes.len()));
    }

    for &byte in bytes {
        if !byte.is_ascii_hexdigit() {
            return Err(format!("Invalid hexadecimal character byte: {}", byte));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_validate_hexadecimal_characters() {

        let valid_hex_chars: &[&[u8]] = &[
            b"00",
            b"1A",
            b"FF",
            b"a0",
            b"7f",
        ];

        for (index, bytes) in valid_hex_chars.iter().enumerate() {
            assert!(super::validate_hexadecimal_char(bytes).is_ok(), "Expected valid: {:?} at index {}", bytes, index);
        }
    }

    #[test]
    fn should_invalidate_hexadecimal_characters() {

        let invalid_hex_chars: &[&[u8]] = &[
            b"",
            b"0",
            b"000",
            b"G1",
            b"1G",
            b"ZZ",
            b"1 ",
        ];

        for (index, bytes) in invalid_hex_chars.iter().enumerate() {
            assert!(super::validate_hexadecimal_char(bytes).is_err(), "Expected invalid: {:?} at index {}", bytes, index);
        }
    }
}