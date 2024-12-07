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
//! @version 1.3
//! @date 2024-11-24

use super::{Token, TokenCategory};
use std::iter::Peekable;
use log::debug;

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
    // Ensure the function starts with a quote
    current_token.push('\''); // Start of string literal
    debug!("Debug: Starting string literal: {}", current_token);

    while let Some(&next_char) = chars.peek() {
        chars.next(); // Consume the character
        debug!("Debug: Consumed character: {}", next_char);
        debug!("Debug: Current token before processing: {}", current_token);

        if next_char == '\'' {
            debug!("Debug: Detected closing quote");
            // Check for escaped quotes ('')
            if chars.peek() == Some(&'\'') {
                debug!("Debug: Detected escaped quote");
                current_token.push(next_char); // Append the first quote
                chars.next(); // Consume the second quote
                current_token.push('\''); // Append the second quote
                debug!("Debug: Updated token with escaped quote: {}", current_token);
            } else {
                // End of string literal
                current_token.push(next_char); // Include the closing quote
                tokens.push(Token::new(
                    &current_token.clone(),
                    TokenCategory::Literal,
                    None,
                ));
                debug!("Debug: Finalized string literal: {}", current_token);
                current_token.clear();
                return;
            }
        } else {
            // Append regular characters to the string literal
            current_token.push(next_char);
            debug!("Debug: Appended to string literal: {}", current_token);
        }
    }

    // Handle unmatched string literal (no closing quote)
    debug!("Debug: Unmatched string literal detected");
    tokens.push(Token::new(
        &current_token.clone(),
        TokenCategory::Literal,
        None,
    ));
    debug!("Debug: Finalized unmatched string literal: {}", current_token);
    current_token.clear();
}








/// Unit tests for `handle_string_literal`.
#[cfg(test)]
mod tests {
    use super::*;
    use log::debug; // For debug logging
    use env_logger; // To initialize the logger

    // Initialize the logger before running tests
    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    /// @test Verifies proper handling of a complete string literal.
    #[test]
    fn test_complete_string_literal() {
        init_logger(); // Enable debug output

        let input = "'complete string'"; // Define the input string
        let mut chars = input.chars().peekable();
        let mut tokens = vec![];
        let mut current_token = String::new();

        debug!("Test input: {}", input);

        // Call the function being tested
        handle_string_literal(&mut chars, &mut tokens, &mut current_token);

        debug!("Tokens generated: {:?}", tokens);

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
        assert_eq!(tokens[0].value, "'unmatched string"); // No closing quote
        assert_eq!(tokens[0].category, TokenCategory::Literal);
    }


    /// @test Verifies handling of an empty string literal.
    #[test]
    fn test_empty_string_literal() {
        init_logger(); // Enable debug output

        let input = "''"; // Define the input string
        let mut chars = input.chars().peekable();
        let mut tokens = vec![];
        let mut current_token = String::new();

        debug!("Test input: {}", input);

        handle_string_literal(&mut chars, &mut tokens, &mut current_token);

        debug!("Tokens generated: {:?}", tokens);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "''");
        assert_eq!(tokens[0].category, TokenCategory::Literal);
    }
}
