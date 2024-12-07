//! @file tokenizer_logic.rs
//! @brief Core tokenization logic for the PL/I Preprocessor.
//!
//! This module provides core functionality for tokenizing PL/I input, 
//! checking for errors in the tokenization process, and validating 
//! preprocessor directives.
//!
//! @details
//! The functions in this module include:
//! - `tokenize_pli`: Tokenizes input into categorized tokens.
//! - `has_tokenizer_error`: Checks for errors in tokenized input.
//! - `is_valid_preprocessor_directive`: Validates directives in tokenized input.
//!
//! @version 1.0
//! @date 2024-11-24

use super::token::finalize_token;
use super::string_literal::handle_string_literal;
use super::directive::handle_directive;
use super::special_char::handle_special_characters;
use super::{Token, TokenCategory};
use log::debug;
use env_logger;

#[cfg(test)]
fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

/// Tokenizes a given PL/I input string into a vector of categorized tokens.
///
/// # Parameters
/// - `input` (`&str`): The PL/I input line to be tokenized.
///
/// # Returns
/// - `Vec<Token>`: A vector of tokens parsed from the input.
pub fn tokenize_pli(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    debug!("Input: {}", input);

    while let Some(c) = chars.next() {
        debug!("Processing character: '{}'", c);
        if c.is_whitespace() {
            // Finalize tokens for whitespace-separated identifiers
            debug!("Encountered whitespace. Finalizing token: '{}'", current_token);
            finalize_token(&mut current_token, &mut tokens, TokenCategory::Identifier);
            continue;
        }

        match c {
            '\'' => {
                // Handle string literals
                debug!("Entering string literal handling");
                handle_string_literal(&mut chars, &mut tokens, &mut current_token);
            }
            '%' => {
                // Handle preprocessor directives
                debug!("Entering directive handling");
                handle_directive(c, &mut chars, &mut current_token, &mut tokens);
            }
            '=' | '#' | '*' | ';' => {
                // Handle special characters
                debug!("Entering special character handling for '{}'", c);
                handle_special_characters(c, &mut chars, &mut current_token, &mut tokens);
            }
            _ if c.is_alphanumeric() || c == '_' => {
                // Collect alphanumeric tokens
                debug!("Appending alphanumeric or underscore: '{}'", c);
                current_token.push(c);
            }
            _ => {
                // Handle remaining special characters
                debug!("Unhandled special character: '{}'", c);
                handle_special_characters(c, &mut chars, &mut current_token, &mut tokens);
            }
        }
    }

    // Finalize any remaining token
    debug!("Finalizing remaining token: '{}'", current_token);
    finalize_token(&mut current_token, &mut tokens, TokenCategory::Identifier);
    debug!("Generated tokens: {:?}", tokens);
    tokens
}

/// Checks for tokenizer errors such as unmatched string literals.
///
/// # Parameters
/// - `tokens` (`&[Token]`): The list of tokens to validate.
///
/// # Returns
/// - `bool`: `true` if any errors are found, `false` otherwise.
pub fn has_tokenizer_error(tokens: &[Token]) -> bool {
    tokens
        .iter()
        .any(|token| token.value.starts_with("'") && !token.value.ends_with("'"))
}

/// Validates the presence of a valid directive.
///
/// # Parameters
/// - `tokens` (`&[Token]`): A slice of tokens to validate.
///
/// # Returns
/// - `bool`: `true` if the first token is a valid directive, `false` otherwise.
pub fn is_valid_preprocessor_directive(tokens: &[Token]) -> bool {
    tokens.get(0).map_or(false, |token| {
        matches!(
            token.value.as_str(),
            "%IF" | "%THEN" | "%ELSE" | "%ENDIF" | "%MACRO" | "%INCLUDE" | "%COMMENT"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @test Verifies that tokenize_pli correctly tokenizes a simple PL/I line.
    #[test]
    fn test_tokenize_pli() {
        let input = "%IF A = B THEN";
        let tokens = tokenize_pli(input);
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].value, "%IF");
        assert_eq!(tokens[1].value, "A");
        assert_eq!(tokens[2].value, "=");
        assert_eq!(tokens[3].value, "B");
        assert_eq!(tokens[4].value, "THEN");
    }

    /// @test Verifies has_tokenizer_error correctly detects unmatched string literals.
    #[test]
    fn test_has_tokenizer_error() {
        let tokens = vec![
            Token::new("'unmatched", TokenCategory::Literal, None),
            Token::new("valid", TokenCategory::Identifier, None),
        ];
        assert!(has_tokenizer_error(&tokens));
    }

    /// @test Verifies is_valid_preprocessor_directive detects valid directives.
    #[test]
    fn test_is_valid_preprocessor_directive() {
        let tokens = vec![Token::new("%IF", TokenCategory::Directive, None)];
        assert!(is_valid_preprocessor_directive(&tokens));

        let tokens = vec![Token::new("INVALID", TokenCategory::Identifier, None)];
        assert!(!is_valid_preprocessor_directive(&tokens));
    }

    #[test]
    fn test_mixed_input_with_invalid_directives() {

        init_logger(); // Initialize logger

        let input = "%INVALID 'string' A = B;";
        let tokens = tokenize_pli(input);
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].value, "%INVALID");
        assert_eq!(tokens[1].value, "'string'");
        assert_eq!(tokens[2].value, "A");
        assert_eq!(tokens[3].value, "=");
        assert_eq!(tokens[4].value, "B");
        assert_eq!(tokens[5].value, ";");
    }

    #[test]
    fn test_multiple_unmatched_strings() {
        let input = "'first 'second'";
        let tokens = tokenize_pli(input);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "'first ");
        assert_eq!(tokens[1].value, "'second'");
    }

    #[test]
    fn test_very_long_input() {
        let input = "%IF ".repeat(1000);
        let tokens = tokenize_pli(&input);
        assert_eq!(tokens.len(), 1000);
        assert!(tokens.iter().all(|t| t.value == "%IF"));
    }

}
