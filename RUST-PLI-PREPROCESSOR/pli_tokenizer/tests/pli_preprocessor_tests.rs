#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// TEST SUITE: PL/I Preprocessor Tests
// -----------------------------------------------------------------------------
// Description:
// This test suite validates the functionality of the PL/I Preprocessor,
// ensuring that tokenization, handling of directives, and other edge cases
// are implemented correctly.
//
// Tests Include:
// - Valid File Tests: Ensure syntactically correct files are tokenized as expected.
// - Invalid File Tests: Validate that errors are detected in malformed input.
// - Edge Cases: Handle unique scenarios like empty lines and special characters.
// - Nested Directives: Verify the proper handling of nested and complex structures.
// - Case Insensitivity: Check that directives and identifiers are case-insensitive.
//
// Usage:
// Run the suite using Cargo:
// $ cargo test --test pli_preprocessor_tests -- --nocapture
//
// Author: Jean-Pierre Sainfeld
// Assistant: ChatGPT
// Company: FirstLink Consulting Services (FLCS)
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use pli_tokenizer::modules::tokenizer::{
        handle_string_literal, is_valid_preprocessor_directive, tokenize_pli, DirectiveCategory,
        Token, TokenCategory,
    };

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Valid File
    // -----------------------------------------------------------------------------
    // Ensures that syntactically valid PL/I files are tokenized correctly.
    // Each line is tokenized, and the resulting tokens are checked to ensure
    // correctness.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_valid_file() {
        let lines = vec![
            "DECLARE X FIXED;",
            "%INCLUDE 'example.pli';",
            "%IF DEBUG = 1 %THEN;",
        ];

        for (line_number, line) in lines.iter().enumerate() {
            let tokens = tokenize_pli(line);
            assert!(
                !tokens.is_empty(),
                "Expected tokens but got none for valid file on line {}",
                line_number + 1
            );
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Invalid File
    // -----------------------------------------------------------------------------
    // Ensures that invalid input lines (e.g., unsupported directives, empty lines,
    // or plain text) are either skipped or flagged as errors.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_invalid_file() {
        let lines = vec!["@INVALID_DIRECTIVE;", "", "Plain text without directives"];

        for (line_number, line) in lines.iter().enumerate() {
            let tokens = tokenize_pli(line);

            if line.trim().is_empty() {
                assert!(
                    tokens.is_empty(),
                    "Expected no tokens for empty line {}",
                    line_number + 1
                );
            } else {
                assert!(
                    !is_valid_preprocessor_directive(&tokens),
                    "Unexpectedly valid tokens for invalid file on line {}",
                    line_number + 1
                );
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Nested Directives
    // -----------------------------------------------------------------------------
    // Tests that nested and deeply nested PL/I directives are tokenized correctly.
    // The test ensures proper recognition and hierarchy of tokens.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_nested_directives() {
        let line = "%IF DEBUG = 1 %THEN; %IF NESTED = 2 %THEN; A = B; %ENDIF; %ENDIF;";
        let tokens = tokenize_pli(line);
        assert_eq!(tokens.len(), 16, "Expected 16 tokens for nested directives");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Case Insensitivity
    // -----------------------------------------------------------------------------
    // Verifies that the tokenizer handles case-insensitive directives and identifiers.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_case_insensitivity() {
        let line = "%if debug = 1 %then;";
        let tokens = tokenize_pli(line);

        assert!(
            tokens.contains(&Token::new(
                "%IF",
                TokenCategory::Directive,
                Some(DirectiveCategory::ControlFlow)
            )),
            "Expected '%IF' token for case-insensitive directive"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Special Characters
    // -----------------------------------------------------------------------------
    // Ensures that special characters (e.g., `;`, `#`, `*`) are tokenized correctly.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_special_characters() {
        let line = "#SPECIAL *CHARS;";
        let tokens = tokenize_pli(line);
        let expected = vec![
            Token::new("#", TokenCategory::Operator, None),
            Token::new("SPECIAL", TokenCategory::Identifier, None),
            Token::new("*", TokenCategory::Operator, None),
            Token::new("CHARS", TokenCategory::Identifier, None),
            Token::new(";", TokenCategory::Separator, None),
        ];
        assert_eq!(tokens, expected, "Failed to tokenize special characters");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Handle String Literal
    // -----------------------------------------------------------------------------
    // Verifies that string literals enclosed in single quotes are tokenized as
    // a single token, even if they contain spaces or special characters.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_handle_string_literal() {
        let mut chars = "'string'".chars().peekable();
        let mut in_string = false;
        let mut current_token = String::new();
        let mut tokens = Vec::new();

        handle_string_literal(
            chars.next().unwrap(),
            &mut chars,
            &mut in_string,
            &mut current_token,
            &mut tokens,
        );

        assert_eq!(
            tokens,
            vec![Token::new("'string'", TokenCategory::Literal, None)],
            "Failed to handle string literal"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Handle Directive
    // -----------------------------------------------------------------------------
    // Ensures that preprocessor directives are tokenized with their correct category.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_handle_directive() {
        let mut chars = "%IFDEBUG".chars().peekable();
        let mut current_token = String::new();
        let mut tokens = Vec::new();

        handle_directive('%', &mut chars, &mut current_token, &mut tokens);

        assert_eq!(
            tokens,
            vec![Token::new(
                "%IFDEBUG",
                TokenCategory::Directive,
                Some(DirectiveCategory::MacroHandling)
            )],
            "Failed to tokenize directive correctly"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Complex Inputs
    // -----------------------------------------------------------------------------
    // Validates tokenization of complex input lines containing directives,
    // string literals, and special characters.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_complex_inputs() {
        let line = "%IF DEBUG = 1 %THEN 'This is a test'; #SPECIAL_CHARS;";
        let tokens = tokenize_pli(line);
        let expected = vec![
            Token::new(
                "%IF",
                TokenCategory::Directive,
                Some(DirectiveCategory::ControlFlow),
            ),
            Token::new("DEBUG", TokenCategory::Identifier, None),
            Token::new("=", TokenCategory::Operator, None),
            Token::new("1", TokenCategory::Literal, None),
            Token::new(
                "%THEN",
                TokenCategory::Directive,
                Some(DirectiveCategory::ControlFlow),
            ),
            Token::new("'This is a test'", TokenCategory::Literal, None),
            Token::new(";", TokenCategory::Separator, None),
            Token::new("#", TokenCategory::Operator, None),
            Token::new("SPECIAL_CHARS", TokenCategory::Identifier, None),
            Token::new(";", TokenCategory::Separator, None),
        ];

        assert_eq!(tokens, expected, "Failed to tokenize complex input");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Unmatched String
    // -----------------------------------------------------------------------------
    // Detects and handles unmatched or improperly closed string literals.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_unmatched_string() {
        let line = "'This is unmatched";
        let tokens = tokenize_pli(line);

        assert!(
            tokens
                .iter()
                .any(|t| t.value.starts_with("'") && !t.value.ends_with("'")),
            "Failed to detect unmatched string literal"
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // TEST: Large File
    // -----------------------------------------------------------------------------
    // Tests tokenization of a large input line repeated multiple times.
    // Ensures performance and correctness for large inputs.
    ////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_large_file() {
        let line = "%IF DEBUG = 1 %THEN;".repeat(1000);
        let tokens = tokenize_pli(&line);

        assert!(!tokens.is_empty(), "Failed to tokenize large file");
    }
}
