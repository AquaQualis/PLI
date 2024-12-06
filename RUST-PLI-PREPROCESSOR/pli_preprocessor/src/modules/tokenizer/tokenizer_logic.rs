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

use super::{Token, TokenCategory};
use super::utils::{finalize_token, handle_string_literal, handle_directive, handle_special_characters};

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
    let mut in_string = false;

    while let Some(c) = chars.next() {
        if c.is_whitespace() && !in_string {
            finalize_token(&mut current_token, &mut tokens);
            continue;
        }

        match c {
            '\'' => handle_string_literal(
                c,
                &mut chars,
                &mut in_string,
                &mut current_token,
                &mut tokens,
            ),
            '%' => handle_directive(c, &mut chars, &mut current_token, &mut tokens),
            '=' | '#' | '*' | ';' => {
                handle_special_characters(c, &mut chars, &mut current_token, &mut tokens)
            }
            _ if c.is_alphanumeric() || c == '_' => current_token.push(c),
            _ => handle_special_characters(c, &mut chars, &mut current_token, &mut tokens),
        }
    }

    finalize_token(&mut current_token, &mut tokens);
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
}
