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
//! @version 1.1
//! @date 2024-11-24

////////////////////////////////////////////////////////////////////////////////
// FUNCTION INVENTORY
// -----------------------------------------------------------------------------
// - `Token` Structure: Represents categorized tokens.
// - `TokenCategory` Enum: Enumerates general token categories.
// - `DirectiveCategory` Enum: Provides finer classification for directives.
// - `get_directive_category`: Retrieves the category for preprocessor directives.
// - `finalize_token`: Finalizes and adds a token to the token list.
////////////////////////////////////////////////////////////////////////////////

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
/// These categories help classify tokens based on their function in the source code.
///
/// # Variants
/// - `Directive`: Tokens representing preprocessor directives.
/// - `Identifier`: Tokens representing identifiers.
/// - `Literal`: Tokens representing string literals or numbers.
/// - `Operator`: Tokens representing operators like `=` or `+`.
/// - `Separator`: Tokens representing separators like `;` or `,`.
/// - `Unknown`: Tokens that cannot be categorized.
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
/// These categories provide finer classification for tokens identified as directives.
///
/// # Variants
/// - `ControlFlow`: Directives related to control flow (e.g., `%IF`, `%THEN`).
/// - `MacroHandling`: Directives related to macros (e.g., `%MACRO`, `%INCLUDE`).
/// - `Conditional`: Directives related to conditional processing (e.g., `%SWITCH`).
/// - `Comment`: Directives representing comments (e.g., `%COMMENT`).
/// - `Other`: Directives not falling into the above categories.
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
/// - `directive` - The directive token as a string slice.
///
/// # Returns
/// - A `DirectiveCategory` enum value representing the category.
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

/// Finalizes the current token by pushing it to the token list and clearing it.
///
/// This function appends the `current_token` to the `tokens` list if it is not empty,
/// and then resets the `current_token` to an empty state.
///
/// # Arguments
/// - `current_token` - A mutable reference to the string representing the current token.
/// - `tokens` - A mutable reference to the vector of tokens to which the finalized token will be added.
/// - `category` - The general category of the token being finalized.
///
/// # Example
/// ```rust
/// let mut tokens = Vec::new();
/// let mut current_token = String::from("example");
/// finalize_token(&mut current_token, &mut tokens, TokenCategory::Identifier);
/// assert_eq!(tokens.len(), 1);
/// assert_eq!(tokens[0].value, "example");
/// ```
pub fn finalize_token(
    current_token: &mut String,
    tokens: &mut Vec<Token>,
    category: TokenCategory,
) {
    if !current_token.is_empty() {
        tokens.push(Token::new(current_token, category, None));
        current_token.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////
// UNIT TESTS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    /// @test Verifies the creation of a `Token` with the `new` constructor.
    #[test]
    fn test_create_new_token() {
        let token = Token::new(
            "%IF",
            TokenCategory::Directive,
            Some(DirectiveCategory::ControlFlow),
        );
        assert_eq!(token.value, "%IF");
        assert_eq!(token.category, TokenCategory::Directive);
        assert_eq!(token.directive_category, Some(DirectiveCategory::ControlFlow));
    }

    /// @test Validates the directive category classification for known directives.
    #[test]
    fn test_get_directive_category() {
        assert_eq!(
            get_directive_category("%IF"),
            DirectiveCategory::ControlFlow
        );
        assert_eq!(
            get_directive_category("%MACRO"),
            DirectiveCategory::MacroHandling
        );
        assert_eq!(
            get_directive_category("%SWITCH"),
            DirectiveCategory::Conditional
        );
        assert_eq!(get_directive_category("%COMMENT"), DirectiveCategory::Comment);
        assert_eq!(get_directive_category("%UNKNOWN"), DirectiveCategory::Other);
    }

    /// @test Ensures `finalize_token` processes a non-empty token correctly.
    #[test]
    fn test_finalize_token_non_empty() {
        let mut tokens = Vec::new();
        let mut current_token = String::from("TOKEN");
        finalize_token(&mut current_token, &mut tokens, TokenCategory::Identifier);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "TOKEN");
    }

    /// @test Ensures `finalize_token` does nothing for an empty token.
    #[test]
    fn test_finalize_token_empty() {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        finalize_token(&mut current_token, &mut tokens, TokenCategory::Identifier);
        assert!(tokens.is_empty());
    }

    /// @test Ensures `finalize_token` handles multiple consecutive calls.
    #[test]
    fn test_finalize_token_multiple_calls() {
        let mut tokens = Vec::new();
        let mut token1 = String::from("TOKEN1");
        let mut token2 = String::from("TOKEN2");
        finalize_token(&mut token1, &mut tokens, TokenCategory::Identifier);
        finalize_token(&mut token2, &mut tokens, TokenCategory::Literal);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "TOKEN1");
        assert_eq!(tokens[1].value, "TOKEN2");
    }
}
