//! Submodule providing the struct `Token`, which represents a Python token.
//!
//! The primary purpose of this module is to provide a string struct that is
//! been validated in such a way that it can be used in a Python project as a
//! class name, function name, variable name, etc.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Token {
    value: String,
}

impl Token {
    /// Create a new token.
    ///
    /// # Arguments
    /// * `value` - The value of the token.
    ///
    /// # Raises
    /// If the value of the token is not a valid Python identifier.
    pub fn new(value: &str) -> Result<Token, String> {
        if !Token::is_valid(value) {
            return Err(format!("{} is not a valid Python identifier", value));
        }
        Ok(Token {
            value: value.to_string(),
        })
    }

    /// Check if a string is a valid Python identifier.
    ///
    /// # Arguments
    /// * `value` - The value to check.
    ///
    /// # Returns
    /// True if the value is a valid Python identifier, false otherwise.
    pub fn is_valid(value: &str) -> bool {
        if value.is_empty() {
            return false;
        }
        if !value.chars().next().unwrap().is_alphabetic() && value.chars().next().unwrap() != '_' {
            return false;
        }
        for c in value.chars() {
            if !c.is_alphanumeric() && c != '_' {
                return false;
            }
        }
        true
    }

    /// Get the value of the token.
    ///
    /// # Returns
    /// The value of the token.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl AsMut<str> for Token {
    fn as_mut(&mut self) -> &mut str {
        &mut self.value
    }
}

impl TryFrom<&str> for Token {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Token::new(value)
    }
}

impl TryFrom<String> for Token {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Token::new(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Token::new("abc").unwrap().value(), "abc");
        assert_eq!(Token::new("abc123").unwrap().value(), "abc123");
        assert_eq!(Token::new("_abc").unwrap().value(), "_abc");
        assert_eq!(
            Token::new("").err().unwrap(),
            " is not a valid Python identifier"
        );
        assert_eq!(
            Token::new("123abc").err().unwrap(),
            "123abc is not a valid Python identifier"
        );
        assert_eq!(
            Token::new("abc!").err().unwrap(),
            "abc! is not a valid Python identifier"
        );
    }
}
