//! @file string_literal.rs
//! @brief Handles tokenization of string literals in the PL/I Preprocessor.
//!
//! This module processes string literals enclosed in single quotes, ensuring
//! proper handling of quoted strings and edge cases like unmatched quotes.
//!
//! @details
//! String literals are sequences of characters enclosed in single quotes (`'`).
//! This module identifies and processes these literals, ensuring proper handling
//! of both complete and unmatched string literals.
//!
//! @version 1.2
//! @date 2024-11-24

use super::{Token, TokenCategory};
use std::iter::Peekable;

/// Handles string literals, ensuring proper tokenization.
///
/// This function processes characters enclosed in single quotes (`'`) as string
/// literals. If an unmatched string literal is encountered, it is still added
/// to the tokens with its current state.
///
/// # Arguments
/// * `chars` - A mutable iterator over the characters of the input string.
/// * `tokens` - A mutable reference to the vector of tokens.
/// * `current_token` - A mutable reference to the current token being processed.
///
/// # Example
/// ```rust
/// let input = "'example string'";
/// let mut chars = input.chars().peekable();
/// let mut tokens = vec![];
/// let mut current_token = String::new();
/// handle_string_literal(&mut chars, &mut tokens, &mut current_token);
/// assert_eq!(tokens[0].value, "'example string'");
/// ```
pub fn handle_string_literal<I>(
    chars: &mut Peekable<I>,
    tokens: &mut Vec<Token>,
    current_token: &mut String,
) where
    I: Iterator<Item = char>,
{
    current_token.push('\''); // Start of string literal
    chars.next(); // Consume the opening quote

    while let Some(&next_char) = chars.peek() {
        current_token.push(next_char);
        chars.next();

        if next_char == '\'' {
            // End of string literal
            tokens.push(Token::new(
                &current_token.clone(),
                TokenCategory::Literal,
                None,
            ));
            current_token.clear();
            return;
        }
    }

    // Handle unmatched string literal (no closing quote)
    tokens.push(Token::new(
        &current_token.clone(),
        TokenCategory::Literal,
        None,
    ));
    current_token.clear();
}




/// Unit tests for `handle_string_literal`.
#[cfg(test)]
mod tests {
    use super::*;

    /// @test Verifies proper handling of a complete string literal.
    #[test]
    fn test_complete_string_literal() {
        let mut chars = "'complete string'".chars().peekable();
        let mut tokens = vec![];
        let mut current_token = String::new();

        handle_string_literal(&mut chars, &mut tokens, &mut current_token);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "'complete string'");
        assert_eq!(tokens[0].category, TokenCategory::Literal);
    }

    /// @test Ensures unmatched string literals are handled gracefully.
    #[test]
    fn test_unmatched_string_literal() {
        let mut chars = "'unmatched string".chars().peekable();
        let mut tokens = vec![];
        let mut current_token = String::new();

        handle_string_literal(&mut chars, &mut tokens, &mut current_token);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "'unmatched string");
        assert_eq!(tokens[0].category, TokenCategory::Literal);
    }

    /// @test Verifies handling of an empty string literal.
    #[test]
    fn test_empty_string_literal() {
        let mut chars = "''".chars().peekable();
        let mut tokens = vec![];
        let mut current_token = String::new();

        handle_string_literal(&mut chars, &mut tokens, &mut current_token);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "''");
        assert_eq!(tokens[0].category, TokenCategory::Literal);
    }
}
