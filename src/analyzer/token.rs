use crate::analyzer::position::Position;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    name: String,
    value: String,
    position: Position,
}

impl Token {
    /// Returns a new token structure using the given arguments.
    ///
    /// # Examples
    /// ```
    /// let pos = scrawlc::Position::default();
    /// let tok = scrawlc::Token::new("exemplum", "exemplum", &pos);
    ///
    /// assert_eq!(tok.name(), "exemplum");
    /// assert_eq!(tok.value(), "exemplum");
    /// assert_eq!(tok.position(), &pos);
    /// ```
    pub fn new(name: &str, value: &str, position: &Position) -> Self {
        Token {
            name: name.to_string(),
            value: value.to_string(),
            position: position.clone(),
        }
    }

    /// Returns the token name.
    ///
    /// # Examples
    /// ```
    /// let pos = scrawlc::Position::default();
    /// let tok = scrawlc::Token::new("exemplum", "exemplum", &pos);
    ///
    /// assert_eq!(tok.name(), "exemplum");
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the token value.
    ///
    /// # Examples
    /// ```
    /// let pos = scrawlc::Position::default();
    /// let tok = scrawlc::Token::new("exemplum", "exemplum", &pos);
    ///
    /// assert_eq!(tok.value(), "exemplum");
    /// ```
    pub fn value(&self) -> &String {
        &self.value
    }

    /// Returns the token position.
    ///
    /// # Examples
    /// ```
    /// let pos = scrawlc::Position::default();
    /// let tok = scrawlc::Token::new("exemplum", "exemplum", &pos);
    ///
    /// assert_eq!(tok.position(), &pos);
    /// ```
    pub fn position(&self) -> &Position {
        &self.position
    }
}

impl fmt::Display for Token {
    /// Formats a string with the token's attributes.
    ///
    /// ```
    /// let pos = scrawlc::Position::default();
    /// let tok = scrawlc::Token::new("exemplum", "exemplum", &pos);
    ///
    /// assert_eq!(tok.to_string(), "<exemplum>(exemplum)@1:1");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>({})@{}", self.name, self.value, self.position)
    }
}
