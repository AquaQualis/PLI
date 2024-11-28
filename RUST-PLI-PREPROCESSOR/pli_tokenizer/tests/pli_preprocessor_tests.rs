////////////////////////////////////////////////////////////////////////////////
// TEST MODULE: PL/I Preprocessor Tests
// -----------------------------------------------------------------------------
// Description:
// This module contains unit tests for the PL/I tokenizer, focusing on the
// functionality provided by the `tokenize_pli` function and its related helpers.
//
// Tests cover:
// - Tokenization of valid and invalid input strings.
// - Case-insensitivity for directives.
// - Handling of special characters and string literals.
//
// Author: Jean-Pierre Sainfeld
// Assistant: ChatGPT
// Company: FirstLink Consulting Services (FLCS)
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use pli_tokenizer::modules::tokenizer::{
        get_directive_category, has_tokenizer_error, is_valid_preprocessor_directive, tokenize_pli,
        DirectiveCategory, Token, TokenCategory,
    };

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: test_case_insensitivity
    // -----------------------------------------------------------------------------
    // Verifies that the tokenizer handles directives in a case-insensitive manner.
    // -----------------------------------------------------------------------------
    #[test]
    fn test_case_insensitivity() {
        let input = "%if debug = 1 %then;";
        let tokens = tokenize_pli(input);

        assert_eq!(tokens.len(), 8, "Expected 8 tokens, got {:?}", tokens);
        assert_eq!(
            tokens[0].value,
            "%IF",
            "Expected '%IF' token for case-insensitive directive"
        );
        assert_eq!(
            tokens[0].category,
            TokenCategory::Directive,
            "Expected 'Directive' category for '%IF'"
        );
        assert_eq!(
            tokens[0].directive_category,
            Some(DirectiveCategory::ControlFlow),
            "Expected 'ControlFlow' directive category for '%IF'"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: test_handle_special_characters
    // -----------------------------------------------------------------------------
    // Verifies that special characters like `;` and `=` are correctly tokenized.
    // -----------------------------------------------------------------------------
    #[test]
    fn test_handle_special_characters() {
        let input = "x = y;";
        let tokens = tokenize_pli(input);

        assert_eq!(tokens.len(), 4, "Expected 4 tokens, got {:?}", tokens);
        assert_eq!(tokens[1].value, "=", "Expected '=' operator token");
        assert_eq!(
            tokens[1].category,
            TokenCategory::Operator,
            "Expected 'Operator' category for '='"
        );
        assert_eq!(tokens[3].value, ";", "Expected ';' separator token");
        assert_eq!(
            tokens[3].category,
            TokenCategory::Separator,
            "Expected 'Separator' category for ';'"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: test_string_literals
    // -----------------------------------------------------------------------------
    // Verifies that string literals are correctly tokenized and unmatched quotes
    // result in errors.
    // -----------------------------------------------------------------------------
    #[test]
    fn test_string_literals() {
        let input = "name = 'John';";
        let tokens = tokenize_pli(input);

        assert_eq!(tokens.len(), 5, "Expected 5 tokens, got {:?}", tokens);
        assert_eq!(
            tokens[2].category,
            TokenCategory::Literal,
            "Expected 'Literal' category for string literal"
        );
        assert_eq!(
            tokens[2].value,
            "'John'",
            "Expected string literal token value to be 'John'"
        );

        let malformed_input = "name = 'John;";
        let malformed_tokens = tokenize_pli(malformed_input);
        assert!(
            has_tokenizer_error(&malformed_tokens),
            "Expected tokenizer error for unmatched string literal"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: test_directive_validation
    // -----------------------------------------------------------------------------
    // Verifies that valid directives are recognized and categorized correctly.
    // -----------------------------------------------------------------------------
    #[test]
    fn test_directive_validation() {
        let input = "%IF DEBUG = 1 %THEN;";
        let tokens = tokenize_pli(input);

        assert!(is_valid_preprocessor_directive(&tokens), "Expected valid directive");

        let invalid_input = "DEBUG = 1;";
        let invalid_tokens = tokenize_pli(invalid_input);
        assert!(
            !is_valid_preprocessor_directive(&invalid_tokens),
            "Expected invalid directive"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: test_edge_cases
    // -----------------------------------------------------------------------------
    // Verifies that edge cases like empty lines and unexpected characters are handled.
    // -----------------------------------------------------------------------------
    #[test]
    fn test_edge_cases() {
        let empty_input = "";
        let empty_tokens = tokenize_pli(empty_input);
        assert_eq!(empty_tokens.len(), 0, "Expected 0 tokens for empty input");

        let unexpected_input = "@invalid";
        let unexpected_tokens = tokenize_pli(unexpected_input);
        assert_eq!(
            unexpected_tokens[0].category,
            TokenCategory::Unknown,
            "Expected 'Unknown' category for '@'"
        );
    }
}
