use crate::values::{io, config_error::ConfigError};

use std::env;

#[derive(Debug)]
pub struct ConfigValue(pub String);

impl ConfigValue {
    pub fn from_env(key: &str) -> io::Result<Self> {
        env::var(key)
            .map(ConfigValue)
            .map_err(|_| ConfigError::from(key).into())
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_config_value_from_env() {
        let expected_key = "TEST";
        let expected_value = "TEST_VALUE";

        env::set_var(expected_key, expected_value);

        let result = ConfigValue::from_env(expected_key);

        env::remove_var(expected_key);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, expected_value);
    }

    #[test]
    fn test_config_value_failure() {
        let expected_key = "TEST";

        let result = ConfigValue::from_env(expected_key);

        assert!(result.is_err());

        let binding = result.unwrap_err();
        let actual_error = binding.downcast_ref::<ConfigError>().unwrap();
        let expected_error = ConfigError::from(expected_key);

        assert_eq!(actual_error.0, expected_error.0);
    }
}