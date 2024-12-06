//! @file directive.rs
//! @brief Handles preprocessor directives in PL/I source code.
//!
//! This module provides functionality for processing and categorizing
//! preprocessor directives in PL/I source code.
//!
//! @author
//! - Jean-Pierre Sainfeld
//! - Assistant: ChatGPT
//!
//! @company FirstLink Consulting Services (FLCS)
//!
//! @version 1.0
//! @date 2024-11-24

use super::token::{Token, TokenCategory, DirectiveCategory};

/// Retrieves the category of a given PL/I preprocessor directive.
///
/// This function maps a directive string (e.g., `%IF`, `%MACRO`) to its corresponding
/// `DirectiveCategory`.
///
/// # Arguments
///
/// * `directive` - The directive string.
///
/// # Returns
///
/// A `DirectiveCategory` indicating the type of the directive.
pub fn get_directive_category(directive: &str) -> DirectiveCategory {
    match directive {
        "%IF" | "%THEN" | "%ELSE" | "%ENDIF" => DirectiveCategory::ControlFlow,
        "%MACRO" | "%INCLUDE" => DirectiveCategory::MacroHandling,
        "%SWITCH" | "%CASE" | "%EVALUATE" => DirectiveCategory::Conditional,
        "%COMMENT" => DirectiveCategory::Comment,
        _ => DirectiveCategory::Other,
    }
}

/// Processes directives in the input and categorizes them.
///
/// This function identifies directives starting with `%`, tokenizes them,
/// and categorizes them appropriately.
///
/// # Arguments
///
/// * `current_char` - The current character, typically `%`.
/// * `chars` - A mutable reference to the character iterator for processing the input.
/// * `current_token` - A mutable reference to the current token string.
/// * `tokens` - A mutable reference to the list of generated tokens.
pub fn handle_directive(
    current_char: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    current_token: &mut String,
    tokens: &mut Vec<Token>,
) {
    current_token.push(current_char);
    while let Some(&next_char) = chars.peek() {
        if next_char.is_alphanumeric() || next_char == '_' {
            current_token.push(next_char);
            chars.next();
        } else {
            break;
        }
    }

    let directive = current_token.to_uppercase();
    let directive_category = get_directive_category(&directive);
    tokens.push(Token::new(
        &directive,
        TokenCategory::Directive,
        Some(directive_category),
    ));
    current_token.clear();
}

/// Unit Test
#[cfg(test)]
mod tests {
    use super::{get_directive_category, DirectiveCategory};

    /// @test test_control_flow_directives
    /// @brief Verifies `get_directive_category` correctly identifies control flow directives.
    ///
    /// This test ensures that directives like `%IF`, `%THEN`, `%ELSE`, and `%ENDIF`
    /// are categorized as `DirectiveCategory::ControlFlow`.
    #[test]
    fn test_control_flow_directives() {
        let directives = vec!["%IF", "%THEN", "%ELSE", "%ENDIF"];
        for directive in directives {
            assert_eq!(
                get_directive_category(directive),
                DirectiveCategory::ControlFlow
            );
        }
    }

    /// @test test_macro_handling_directives
    /// @brief Verifies `get_directive_category` correctly identifies macro handling directives.
    ///
    /// This test ensures that directives like `%MACRO` and `%INCLUDE`
    /// are categorized as `DirectiveCategory::MacroHandling`.
    #[test]
    fn test_macro_handling_directives() {
        let directives = vec!["%MACRO", "%INCLUDE"];
        for directive in directives {
            assert_eq!(
                get_directive_category(directive),
                DirectiveCategory::MacroHandling
            );
        }
    }

    /// @test test_conditional_directives
    /// @brief Verifies `get_directive_category` correctly identifies conditional directives.
    ///
    /// This test ensures that directives like `%SWITCH`, `%CASE`, and `%EVALUATE`
    /// are categorized as `DirectiveCategory::Conditional`.
    #[test]
    fn test_conditional_directives() {
        let directives = vec!["%SWITCH", "%CASE", "%EVALUATE"];
        for directive in directives {
            assert_eq!(
                get_directive_category(directive),
                DirectiveCategory::Conditional
            );
        }
    }

    /// @test test_comment_directives
    /// @brief Verifies `get_directive_category` correctly identifies comment directives.
    ///
    /// This test ensures that `%COMMENT` is categorized as `DirectiveCategory::Comment`.
    #[test]
    fn test_comment_directives() {
        assert_eq!(
            get_directive_category("%COMMENT"),
            DirectiveCategory::Comment
        );
    }

    /// @test test_unknown_directives
    /// @brief Verifies `get_directive_category` correctly identifies unknown directives.
    ///
    /// This test ensures that unrecognized directives are categorized as `DirectiveCategory::Other`.
    #[test]
    fn test_unknown_directives() {
        let directives = vec!["%UNKNOWN", "%INVALID", "%RANDOM"];
        for directive in directives {
            assert_eq!(
                get_directive_category(directive),
                DirectiveCategory::Other
            );
        }
    }
}
