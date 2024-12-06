//! @file token.rs
//! @brief Defines the `Token` structure and related enums for categorizing PL/I source tokens.
//!
//! This module provides the `Token` structure, which represents the smallest
//! meaningful units (tokens) in the tokenization process. It also includes
//! enums for classifying tokens into general and directive-specific categories.
//!
//! @details
//! Tokens are categorized based on their role in the source code, such as directives,
//! literals, identifiers, operators, or separators. Special handling is provided
//! for PL/I preprocessor directives and their subcategories.
//!
//! @author
//! - Jean-Pierre Sainfeld
//! - Assistant: ChatGPT
//!
//! @company FirstLink Consulting Services (FLCS)
//!
//! @version 1.0
//! @date 2024-11-24

/// Represents a token in the PL/I tokenizer.
///
/// A `Token` consists of its raw text value, a general category, and an optional
/// specific category if it is a directive.
///
/// # Fields
/// * `value` - The raw text of the token.
/// * `category` - The general category of the token, represented by `TokenCategory`.
/// * `directive_category` - An optional specific category if the token is a directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String,
    pub category: TokenCategory,
    pub directive_category: Option<DirectiveCategory>,
}

impl Token {
    /// Creates a new `Token` instance.
    ///
    /// # Arguments
    /// * `value` - The raw text of the token.
    /// * `category` - The general category of the token.
    /// * `directive_category` - An optional specific category if the token is a directive.
    ///
    /// # Returns
    /// * A new `Token` instance.
    ///
    /// # Example
    /// ```rust
    /// use pli_preprocessor::modules::tokenizer::token::{Token, TokenCategory, DirectiveCategory};
    ///
    /// let token = Token::new(
    ///     "%IF",
    ///     TokenCategory::Directive,
    ///     Some(DirectiveCategory::ControlFlow),
    /// );
    ///
    /// assert_eq!(token.value, "%IF");
    /// assert_eq!(token.category, TokenCategory::Directive);
    /// assert_eq!(token.directive_category, Some(DirectiveCategory::ControlFlow));
    /// ```
    pub fn new(
        value: &str,
        category: TokenCategory,
        directive_category: Option<DirectiveCategory>,
    ) -> Self {
        Self {
            value: value.to_string(),
            category,
            directive_category,
        }
    }
}

/// Enumerates general categories for tokens.
///
/// These categories help classify tokens based on their function in the
/// source code. Examples include directives, literals, operators, and separators.
///
/// # Variants
/// * `Directive` - Tokens that represent preprocessor directives.
/// * `Identifier` - Tokens that represent identifiers.
/// * `Literal` - Tokens that represent string literals or numbers.
/// * `Operator` - Tokens that represent operators like `=` or `+`.
/// * `Separator` - Tokens that represent separators like `;` or `,`.
/// * `Unknown` - Tokens that cannot be categorized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenCategory {
    Directive,
    Identifier,
    Literal,
    Operator,
    Separator,
    Unknown,
}

/// Enumerates specific categories for preprocessor directives.
///
/// These categories are used to provide finer classification for tokens
/// identified as directives.
///
/// # Variants
/// * `ControlFlow` - Directives related to control flow (e.g., `%IF`, `%THEN`).
/// * `MacroHandling` - Directives related to macros (e.g., `%MACRO`, `%INCLUDE`).
/// * `Conditional` - Directives related to conditional processing (e.g., `%SWITCH`).
/// * `Comment` - Directives that represent comments (e.g., `%COMMENT`).
/// * `Other` - Any other directives not falling into the above categories.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DirectiveCategory {
    ControlFlow,
    MacroHandling,
    Conditional,
    Comment,
    Other,
}

/// Retrieves the category of a given PL/I preprocessor directive.
///
/// This function matches a directive token with its specific category.
///
/// # Arguments
/// * `directive` - The directive token as a string slice.
///
/// # Returns
/// * A `DirectiveCategory` enum value representing the category.
///
/// # Example
/// ```rust
/// use pli_preprocessor::modules::tokenizer::token::{get_directive_category, DirectiveCategory};
///
/// let category = get_directive_category("%IF");
/// assert_eq!(category, DirectiveCategory::ControlFlow);
/// ```
pub fn get_directive_category(directive: &str) -> DirectiveCategory {
    match directive {
        "%IF" | "%THEN" | "%ELSE" | "%ENDIF" => DirectiveCategory::ControlFlow,
        "%MACRO" | "%INCLUDE" => DirectiveCategory::MacroHandling,
        "%SWITCH" | "%CASE" | "%EVALUATE" => DirectiveCategory::Conditional,
        "%COMMENT" => DirectiveCategory::Comment,
        _ => DirectiveCategory::Other,
    }
}


/// Unit Test
#[cfg(test)]
mod tests {
    use super::{DirectiveCategory, Token, TokenCategory};

    /// @test test_create_new_token
    /// @brief Verifies the correct creation of a `Token` instance.
    ///
    /// This test ensures that the `Token::new` method correctly initializes
    /// the `value`, `category`, and `directive_category` fields.
    #[test]
    fn test_create_new_token() {
        let token = Token::new(
            "example",
            TokenCategory::Identifier,
            Some(DirectiveCategory::ControlFlow),
        );

        assert_eq!(token.value, "example"); // Check if the value matches.
        assert_eq!(token.category, TokenCategory::Identifier); // Verify the category.
        assert_eq!(token.directive_category, Some(DirectiveCategory::ControlFlow)); // Verify directive category.
    }

    /// @test test_token_equality
    /// @brief Verifies equality comparison for `Token` instances.
    ///
    /// This test ensures that two tokens with identical fields
    /// are considered equal.
    #[test]
    fn test_token_equality() {
        let token1 = Token::new(
            "example",
            TokenCategory::Identifier,
            Some(DirectiveCategory::ControlFlow),
        );
        let token2 = Token::new(
            "example",
            TokenCategory::Identifier,
            Some(DirectiveCategory::ControlFlow),
        );
        assert_eq!(token1, token2); // Tokens with the same data should be equal.
    }

    /// @test test_token_inequality
    /// @brief Verifies inequality comparison for `Token` instances.
    ///
    /// This test ensures that two tokens with differing fields
    /// are not considered equal.
    #[test]
    fn test_token_inequality() {
        let token1 = Token::new(
            "example1",
            TokenCategory::Identifier,
            Some(DirectiveCategory::ControlFlow),
        );
        let token2 = Token::new(
            "example2",
            TokenCategory::Identifier,
            Some(DirectiveCategory::ControlFlow),
        );
        assert_ne!(token1, token2); // Tokens with different data should not be equal.
    }
}
