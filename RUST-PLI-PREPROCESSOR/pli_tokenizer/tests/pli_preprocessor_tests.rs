////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Preprocessor Tests
// -----------------------------------------------------------------------------
// Description:
// This module contains unit tests for the PL/I tokenizer, focusing on edge 
// cases, functionality validation, and adherence to preprocessor directives.
//
// Features:
// - Validation of tokenization process for various inputs.
// - Edge case handling, including nested directives and malformed inputs.
// - Case-insensitivity verification for directives.
//
// Author: Jean-Pierre Sainfeld
// Assistant: ChatGPT
// Company: FirstLink Consulting Services (FLCS)
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

use pli_tokenizer::modules::tokenizer::{
    has_tokenizer_error, is_valid_preprocessor_directive, tokenize_pli, Token, TokenCategory,
};

////////////////////////////////////////////////////////////////////////////////
// TEST: test_case_insensitivity
// -----------------------------------------------------------------------------
// Validates case insensitivity for PL/I preprocessor directives.
//
// # Description:
// Ensures that directives such as `%IF` are correctly identified regardless of 
// their case (e.g., `%if`, `%If`, `%IF`). 
//
// # Test Cases:
// - Input: `%if debug = 1 %then;`
// - Expected Output: `%IF` categorized as `Directive`.
//
// # See Also:
// - `tokenize_pli`: Main tokenization function.
////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_case_insensitivity() {
    let input = "%if debug = 1 %then;";
    let tokens = tokenize_pli(input);

    println!("Generated Tokens: {:?}", tokens);

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
}

////////////////////////////////////////////////////////////////////////////////
// TEST: test_handle_directive
// -----------------------------------------------------------------------------
// Validates the handling of valid preprocessor directives.
//
// # Description:
// Ensures that known directives are categorized appropriately, with their
// `DirectiveCategory` correctly assigned.
//
// # Test Cases:
// - Input: `%IF DEBUG = 1 %THEN;`
// - Expected Output: `%IF` as `Directive` with category `ControlFlow`.
//
// # See Also:
// - `handle_directive`: Directive handler.
////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_handle_directive() {
    let input = "%IF DEBUG = 1 %THEN;";
    let tokens = tokenize_pli(input);

    println!("Generated Tokens: {:?}", tokens);

    assert_eq!(
        tokens[0].value,
        "%IF",
        "Expected '%IF' token as a directive"
    );
    assert_eq!(
        tokens[0].directive_category,
        Some(pli_tokenizer::modules::tokenizer::DirectiveCategory::ControlFlow),
        "Expected 'ControlFlow' directive category for '%IF'"
    );
}

////////////////////////////////////////////////////////////////////////////////
// TEST: test_nested_directives
// -----------------------------------------------------------------------------
// Validates the tokenizer's ability to handle nested directives.
//
// # Description:
// Ensures proper tokenization of nested structures in PL/I preprocessor code.
//
// # Test Cases:
// - Input: `%IF %MACRO(DEBUG) %THEN;`
// - Expected Output: Correct categorization of nested directives.
//
// # See Also:
// - `tokenize_pli`: Main tokenization function.
////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_nested_directives() {
    let input = "%IF %MACRO(DEBUG) %THEN;";
    let tokens = tokenize_pli(input);

    println!("Generated Tokens: {:?}", tokens);

    assert_eq!(
        tokens[0].value,
        "%IF",
        "Expected '%IF' as the outer directive"
    );
    assert_eq!(
        tokens[1].value,
        "%MACRO",
        "Expected '%MACRO' as the nested directive"
    );
    assert_eq!(
        tokens[3].value,
        "%THEN",
        "Expected '%THEN' as the closing directive"
    );
}

////////////////////////////////////////////////////////////////////////////////
// TEST: test_complex_inputs
// -----------------------------------------------------------------------------
// Validates the tokenizer's handling of complex PL/I inputs.
//
// # Description:
// Ensures correct handling of mixed input with directives, literals, and
// special characters.
//
// # Test Cases:
// - Input: `%IF 'string' = "other_string" %THEN;`
// - Expected Output: Tokens for directives, literals, and operators.
//
// # See Also:
// - `tokenize_pli`: Main tokenization function.
////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_complex_inputs() {
    let input = "%IF 'string' = \"other_string\" %THEN;";
    let tokens = tokenize_pli(input);

    println!("Generated Tokens: {:?}", tokens);

    assert_eq!(
        tokens[0].value,
        "%IF",
        "Expected '%IF' as the directive"
    );
    assert_eq!(
        tokens[1].value,
        "'string'",
        "Expected 'string' as a literal"
    );
    assert_eq!(
        tokens[3].value,
        "\"other_string\"",
        "Expected \"other_string\" as a literal"
    );
    assert_eq!(
        tokens[4].value,
        "%THEN",
        "Expected '%THEN' as the closing directive"
    );
}

////////////////////////////////////////////////////////////////////////////////
// TEST: test_has_tokenizer_error
// -----------------------------------------------------------------------------
// Validates detection of tokenizer errors such as unmatched string literals.
//
// # Description:
// Ensures that errors in the input, like unmatched quotes, are correctly flagged.
//
// # Test Cases:
// - Input: `'unmatched string`
// - Expected Output: Error detected.
//
// # See Also:
// - `has_tokenizer_error`: Error detection function.
////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_has_tokenizer_error() {
    let input = "'unmatched string";
    let tokens = tokenize_pli(input);

    assert!(
        has_tokenizer_error(&tokens),
        "Expected tokenizer error for unmatched string"
    );
}
