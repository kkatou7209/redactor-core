use crate::util::check::is_valid_name_char;

/// PDF Name object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    /// The bytes of the name.
    bytes: Vec<u8>,
}

impl Name {
    
    /// Creates a new `Name` from the given bytes.
    pub fn new(bytes: &[u8]) -> Self {
        
        if !bytes.iter().all(is_valid_name_char) {
            panic!("name contains invalid characters");
        }

        Self { bytes: bytes.to_vec() }
    }

    /// Returns the string representation of the Name.
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes).expect("unexpected bytes given.")
    }

    /// Returns the byte representation of the Name.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

mod tests {
    use super::Name;

    #[test]
    fn should_create_valid_name() {
        let name = Name::new(b"ExampleName");
        assert_eq!(name.as_str(), "ExampleName");
        assert_eq!(name.as_bytes(), b"ExampleName");
    }

    #[test]
    #[should_panic(expected = "name contains invalid characters")]
    fn should_panic_when_creating_name_with_invalid_characters() {
        let _name = Name::new(b"Invalid %Name!");
    }
}