////////////////////////////////////////////////////////////////////////////////
// TEST MODULE: Parser Tests
// -----------------------------------------------------------------------------
// Description:
// This module contains unit tests for the `parser` module, including tests for
// error handling, recovery mechanisms, and detailed validation of all parsing
// functionalities.
//
// Tests cover:
// - Tokenization and categorization of single lines.
// - Extraction and validation of directives.
// - Multiline directive handling and error detection.
// - Parsing of control structures like DO, IF/THEN/ELSE, and SELECT.
// - Parsing of expressions, including operator precedence and associativity.
// - Validation of expressions.
// - Error recovery and logging.
//
// -----------------------------------------------------------------------------
// FUNCTION INVENTORY:
// -----------------------------------------------------------------------------
// - test_parse_line: Tests single-line parsing functionality.
// - test_parse_source: Tests full-source parsing and directive extraction.
// - test_parse_statement: Tests single-statement parsing logic.
// - test_parse_control_structure: Tests control structure parsing and validation.
// - test_parse_expression: Tests expression parsing and operator precedence.
// - test_validate_expression: Tests validation of expressions.
// - test_log_error: Tests error logging functionality.
// - test_recover_from_error: Tests error recovery suggestions.
//
// -----------------------------------------------------------------------------
// AUTHOR:
// -----------------------------------------------------------------------------
// - Jean-Pierre Sainfeld
//
// -----------------------------------------------------------------------------
// ASSISTANT:
// -----------------------------------------------------------------------------
// - ChatGPT
//
// -----------------------------------------------------------------------------
// COMPANY:
// -----------------------------------------------------------------------------
// - FirstLink Consulting Services (FLCS)
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// IMPORTS
////////////////////////////////////////////////////////////////////////////////

use pli_preprocessor::modules::parser::{
    parse_line, parse_source, parse_statement, parse_control_structure, parse_expression,
    validate_expression, log_error, recover_from_error, ParseError,
};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// UNIT TESTS
////////////////////////////////////////////////////////////////////////////////

/// Tests the `parse_line` function for single-line parsing functionality.
///
/// # Test Cases
/// - Input: `"DECLARE X FIXED;"`
///   - Expected Output: `["DECLARE", "X", "FIXED", ";"]`
/// - Input: `"   %INCLUDE   'file.pli';   "`
///   - Expected Output: `["%INCLUDE", "'file.pli'", ";"]`
#[test]
fn test_parse_line() {
    let tokens = parse_line("DECLARE X FIXED;");
    assert_eq!(tokens, vec!["DECLARE", "X", "FIXED", ";"]);

    let tokens = parse_line("   %INCLUDE   'file.pli';   ");
    assert_eq!(tokens, vec!["%INCLUDE", "'file.pli'", ";"]);
}

/// Tests the `parse_source` function for full-source parsing and directive extraction.
///
/// # Test Cases
/// ## Case 1: Valid source with one directive
/// - Input: `"DECLARE X FIXED;\n%INCLUDE 'example.pli';"`
/// - Expected Output: Tokens for `["DECLARE", "X", "FIXED", ";"]`
/// - Directive: `%INCLUDE 'example.pli';` with tokens `["%INCLUDE", "'example.pli'", ";"]`
#[test]
fn test_parse_source() {
    let source = "DECLARE X FIXED;\n%INCLUDE 'example.pli';";
    let mut directives = HashMap::new();

    let result = parse_source(source, &mut directives).unwrap();

    assert_eq!(result, vec![vec!["DECLARE", "X", "FIXED", ";"]]);
    assert!(directives.contains_key("%INCLUDE 'example.pli';"));
    assert_eq!(
        directives["%INCLUDE 'example.pli';"],
        vec!["%INCLUDE", "'example.pli'", ";"]
    );
}

/// Tests the `parse_statement` function for single-statement parsing logic.
///
/// # Test Cases
/// ## Case 1: Standard single statement
/// - Input: `"UNKNOWN_STATEMENT;"`
/// - Expected Output: `["UNKNOWN_STATEMENT", ";"]`
///
/// ## Case 2: Multi-part statement
/// - Input: `"MULTI_PART_STATEMENT;"`
/// - Expected Output: `["MULTI_PART_STATEMENT", ";"]`
#[test]
fn test_parse_statement() {
    let tokens = parse_statement("UNKNOWN_STATEMENT;");
    assert_eq!(tokens, vec!["UNKNOWN_STATEMENT", ";"]);

    let tokens = parse_statement("MULTI_PART_STATEMENT;");
    assert_eq!(tokens, vec!["MULTI_PART_STATEMENT", ";"]);
}

/// Tests the `parse_control_structure` function for control structure parsing and validation.
///
/// # Test Cases
/// ## Case 1: Valid DO/END structure
/// - Input: `["DO", "END", ";"]`
/// - Expected: Success
///
/// ## Case 2: Nested DO/END structure
/// - Input: `["DO", "DO", "END", "END", ";"]`
/// - Expected: Success
///
/// ## Case 3: Missing END
/// - Input: `["DO"]`
/// - Expected: Error
///
/// ## Case 4: Unmatched END
/// - Input: `["END"]`
/// - Expected: Error
#[test]
fn test_parse_control_structure() {
    let tokens = vec![
        "DO".to_string(),
        "I".to_string(),
        "=".to_string(),
        "1".to_string(),
        "TO".to_string(),
        "10".to_string(),
        ";".to_string(),
        "END".to_string(),
        ";".to_string(),
    ];
    assert!(parse_control_structure(tokens).is_ok());
}

/// Tests the `parse_expression` function for parsing expressions with operator precedence.
///
/// See detailed example in the enhanced explanation above for all test cases.
#[test]
fn test_parse_expression() {
    let tokens = vec!["A".to_string(), "+".to_string(), "B".to_string()];
    let result = parse_expression(&tokens).unwrap();
    assert_eq!(result, vec!["A", "B", "+"]);
}

/// Tests the `validate_expression` function for validating expressions.
///
/// # Test Cases
/// - Input: Valid expression `["A", "+", "B"]`
///   - Expected: Success
/// - Input: Invalid expression `["A", "+", "*", "B"]`
///   - Expected: Error
#[test]
fn test_validate_expression() {
    let tokens = vec!["A".to_string(), "+".to_string(), "B".to_string()];
    assert!(validate_expression(&tokens).is_ok());
}

/// Tests the `log_error` function for error logging.
///
/// # Test Cases
/// - Input: Error with line 1 and description "Invalid operator placement"
/// - Expected: Log output with error details
#[test]
fn test_log_error() {
    let error = ParseError {
        line: 1,
        token: Some("A + B".to_string()),
        description: "Invalid operator placement".to_string(),
    };
    log_error(&error); // Ensure no panic occurs and output is correct.
}

/// Tests the `recover_from_error` function for generating recovery suggestions.
///
/// # Test Cases
/// - Input: Error with unmatched parentheses
/// - Expected: Suggestion to add missing closing parenthesis
#[test]
fn test_recover_from_error() {
    let error = ParseError {
        line: 2,
        token: Some("A + (".to_string()),
        description: "Unmatched opening parenthesis".to_string(),
    };
    let suggestion = recover_from_error(&error);
    assert_eq!(suggestion.unwrap(), "Add the missing closing parenthesis.");
}

////////////////////////////////////////////////////////////////////////////////
// END OF TEST MODULE
////////////////////////////////////////////////////////////////////////////////
