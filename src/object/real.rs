/// A PDF Real object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Real {
    /// The string representation of the real number.
    value: String,
}

impl Real {
    
    /// Creates a new `Real` from the given bytes.
    pub fn new(value: f64) -> Result<Self, String> {

        if value.is_infinite() || value.is_nan() {
            return Err(format!("Invalid real number: value = {:?}", value));
        }

        let value = format!("{}", value);

        Ok(Self { value, })
    }

    /// Returns the value of the Real as f64.
    pub fn as_f64(&self) -> f64 {

        self.value.parse::<f64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Real;

    #[test]
    fn should_returns_valid_value() {

        let real = Real::new(3.14159).unwrap();
        assert_eq!(real.as_f64(), 3.14159);

        let real = Real::new(-0.001).unwrap();
        assert_eq!(real.as_f64(), -0.001);

        let real = Real::new(0.0000000001).unwrap();
        assert_eq!(real.as_f64(), 0.0000000001);
    }
}