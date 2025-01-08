use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ConfigError(pub String);

impl std::error::Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} must be set!", self.0)
    }
}

impl From<&str> for ConfigError {
    fn from(error: &str) -> Self {
        ConfigError(error.to_string())
    }
}

#[cfg(test)]
mod config_tests {
    use crate::values::config_error::ConfigError;

    #[test]
    fn test_config_error_from_string() {
        let input_str: &str = "Test";
        let expected_string: String = "Test".to_string();

        assert_eq!(ConfigError::from(input_str), ConfigError(expected_string));
    }

    #[test]
    fn test_config_error_formatted_display() {
        let input: String = "Test".to_string();
        let actual: String = format!("{}", ConfigError(input));
        let expected: String = "Test must be set!".to_string();

        assert_eq!(actual, expected);
    }
}