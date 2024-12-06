//! @file special_char.rs
//! @brief Handles special character processing in PL/I source code.
//!
//! This module provides functionality for identifying, categorizing,
//! and handling special characters during tokenization of PL/I source code.
//!
//! @author
//! - Jean-Pierre Sainfeld
//! - Assistant: ChatGPT
//!
//! @company FirstLink Consulting Services (FLCS)
//!
//! @version 1.0
//! @date 2024-11-24

use super::token::{Token, TokenCategory};

/// Handles special characters in the input and assigns appropriate categories.
///
/// This function processes characters such as `=` or `;` and categorizes them
/// as operators, separators, or unknown symbols. It also finalizes the current
/// token being constructed before processing the special character.
///
/// # Arguments
///
/// * `c` - The special character being processed.
/// * `_chars` - A mutable reference to the character iterator (unused in this function).
/// * `current_token` - A mutable reference to the current token being constructed.
/// * `tokens` - A mutable reference to the list of tokens.
///
/// # Example
///
/// ```rust
/// let mut current_token = String::new();
/// let mut tokens = Vec::new();
/// handle_special_characters('=', &mut current_token, &mut tokens);
/// assert_eq!(tokens[0].value, "=");
/// assert_eq!(tokens[0].category, TokenCategory::Operator);
/// ```
pub fn handle_special_characters(
    c: char,
    _chars: &mut std::iter::Peekable<std::str::Chars>,
    current_token: &mut String,
    tokens: &mut Vec<Token>,
) {
    // Finalize the current token before handling the special character.
    finalize_token(current_token, tokens);

    // Categorize the special character and create a token.
    let token_category = match c {
        '=' | '#' | '*' => TokenCategory::Operator,
        ';' => TokenCategory::Separator,
        _ => TokenCategory::Unknown,
    };

    // Add the special character as a token.
    tokens.push(Token::new(&c.to_string(), token_category, None));
}

/// Finalizes the current token and adds it to the token list.
///
/// This function ensures that any partially constructed token is added to
/// the list of tokens before processing a new token.
///
/// # Arguments
///
/// * `current_token` - A mutable reference to the token string being finalized.
/// * `tokens` - A mutable reference to the list of tokens.
///
/// # Example
///
/// ```rust
/// let mut current_token = String::from("TOKEN");
/// let mut tokens = Vec::new();
/// finalize_token(&mut current_token, &mut tokens);
/// assert_eq!(tokens[0].value, "TOKEN");
/// assert_eq!(current_token.is_empty(), true);
/// ```
pub fn finalize_token(current_token: &mut String, tokens: &mut Vec<Token>) {
    if !current_token.is_empty() {
        tokens.push(Token::new(
            &current_token.to_uppercase(),
            TokenCategory::Identifier,
            None,
        ));
        current_token.clear();
    }
}

/// Unit Test

#[cfg(test)]
mod tests {
    use super::handle_special_characters;
    use super::{finalize_token, Token, TokenCategory};

    /// @test test_single_special_character
    /// @brief Verifies that single special characters are correctly tokenized.
    ///
    /// This test ensures that characters such as `=` and `;` are identified as their respective
    /// token categories (e.g., `Operator`, `Separator`).
    #[test]
    fn test_single_special_character() {
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        handle_special_characters('=', &mut "".chars().peekable(), &mut current_token, &mut tokens);
        assert_eq!(
            tokens,
            vec![Token::new("=", TokenCategory::Operator, None)]
        );

        tokens.clear();
        handle_special_characters(';', &mut "".chars().peekable(), &mut current_token, &mut tokens);
        assert_eq!(
            tokens,
            vec![Token::new(";", TokenCategory::Separator, None)]
        );
    }

    /// @test test_mixed_special_characters
    /// @brief Verifies handling of multiple consecutive special characters.
    ///
    /// This test ensures that characters like `=*;` are tokenized correctly
    /// as separate tokens.
    #[test]
    fn test_mixed_special_characters() {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut input = "=*;".chars().peekable();

        while let Some(c) = input.next() {
            handle_special_characters(c, &mut input, &mut current_token, &mut tokens);
        }

        assert_eq!(
            tokens,
            vec![
                Token::new("=", TokenCategory::Operator, None),
                Token::new("*", TokenCategory::Operator, None),
                Token::new(";", TokenCategory::Separator, None)
            ]
        );
    }

    /// @test test_special_characters_with_text
    /// @brief Verifies correct handling when special characters are mixed with text.
    ///
    /// This test ensures that tokens like `SET=A;` result in the expected sequence:
    /// `SET`, `=`, `A`, `;`.
    #[test]
    fn test_special_characters_with_text() {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut input = "SET=A;".chars().peekable();

        while let Some(c) = input.next() {
            if c.is_alphanumeric() || c == '_' {
                current_token.push(c);
            } else {
                handle_special_characters(c, &mut input, &mut current_token, &mut tokens);
            }
        }
        finalize_token(&mut current_token, &mut tokens);

        assert_eq!(
            tokens,
            vec![
                Token::new("SET", TokenCategory::Identifier, None),
                Token::new("=", TokenCategory::Operator, None),
                Token::new("A", TokenCategory::Identifier, None),
                Token::new(";", TokenCategory::Separator, None)
            ]
        );
    }

    /// @test test_finalize_with_special_characters
    /// @brief Verifies that tokens are finalized correctly before adding special characters.
    ///
    /// This test ensures that text preceding a special character is tokenized
    /// separately from the special character itself.
    #[test]
    fn test_finalize_with_special_characters() {
        let mut tokens = Vec::new();
        let mut current_token = String::from("TEST");

        handle_special_characters(';', &mut "".chars().peekable(), &mut current_token, &mut tokens);
        assert_eq!(
            tokens,
            vec![
                Token::new("TEST", TokenCategory::Identifier, None),
                Token::new(";", TokenCategory::Separator, None)
            ]
        );
    }
}
