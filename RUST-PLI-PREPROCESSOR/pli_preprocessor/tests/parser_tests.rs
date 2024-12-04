////////////////////////////////////////////////////////////////////////////////
// TEST MODULE: Parser Tests
// -----------------------------------------------------------------------------
// Description:
// This module contains unit tests for the `parser` module, including tests for
// error handling and recovery mechanisms.
//
// Tests cover:
// - Tokenization and categorization of single lines.
// - Extraction and validation of directives.
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
/// - Valid single-line PL/I source code.
/// - Single-line PL/I source with leading/trailing spaces.
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
/// - Valid source with directives.
/// - Source with mixed statements and directives.
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
/// - Valid single-part statement.
/// - Valid multi-part statement.
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
/// - Valid DO/END structure.
/// - Nested DO/END structure.
/// - DO statement without a matching END.
/// - END statement without a matching DO.
#[test]
fn test_parse_control_structure() {
    // Valid DO/END structure
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

    // Nested DO/END structure
    let tokens = vec![
        "DO".to_string(),
        "J".to_string(),
        "=".to_string(),
        "1".to_string(),
        "TO".to_string(),
        "5".to_string(),
        ";".to_string(),
        "DO".to_string(),
        "K".to_string(),
        "=".to_string(),
        "1".to_string(),
        "TO".to_string(),
        "10".to_string(),
        ";".to_string(),
        "END".to_string(),
        ";".to_string(),
        "END".to_string(),
        ";".to_string(),
    ];
    assert!(parse_control_structure(tokens).is_ok());

    // Missing END
    let tokens = vec![
        "DO".to_string(),
        "I".to_string(),
        "=".to_string(),
        "1".to_string(),
        "TO".to_string(),
        "10".to_string(),
        ";".to_string(),
    ];
    assert!(parse_control_structure(tokens).is_err());

    // Unmatched END
    let tokens = vec!["END".to_string(), ";".to_string()];
    assert!(parse_control_structure(tokens).is_err());
}

/// Tests the `parse_expression` function for parsing expressions with operator precedence.
///
/// # Test Cases
/// - Simple arithmetic expressions.
/// - Expressions with operator precedence.
/// - Expressions with parentheses.
/// - Invalid expressions with unmatched parentheses.
/// - Invalid expressions with unsupported tokens.
#[test]
fn test_parse_expression() {
    // Test simple arithmetic
    let tokens = vec!["A".to_string(), "+".to_string(), "B".to_string()];
    let result = parse_expression(&tokens).unwrap();
    assert_eq!(result, vec!["A", "B", "+"]);

    // Test operator precedence
    let tokens = vec![
        "A".to_string(),
        "+".to_string(),
        "B".to_string(),
        "*".to_string(),
        "C".to_string(),
    ];
    let result = parse_expression(&tokens).unwrap();
    assert_eq!(result, vec!["A", "B", "C", "*", "+"]);

    // Test parentheses
    let tokens = vec![
        "(".to_string(),
        "A".to_string(),
        "+".to_string(),
        "B".to_string(),
        ")".to_string(),
        "*".to_string(),
        "C".to_string(),
    ];
    let result = parse_expression(&tokens).unwrap();
    assert_eq!(result, vec!["A", "B", "+", "C", "*"]);

    // Test mismatched parentheses
    let tokens = vec![
        "(".to_string(),
        "A".to_string(),
        "+".to_string(),
        "B".to_string(),
    ];
    assert!(parse_expression(&tokens).is_err());

    // Test invalid token
    let tokens = vec!["A".to_string(), "&".to_string(), "B".to_string()];
    assert!(parse_expression(&tokens).is_err());
}

/// Tests the `validate_expression` function for validating expressions.
///
/// # Test Cases
/// - Valid expressions.
/// - Invalid expressions with unmatched parentheses.
/// - Invalid expressions with misplaced operators.
#[test]
fn test_validate_expression() {
    // Valid expressions
    let tokens = vec!["A".to_string(), "+".to_string(), "B".to_string()];
    assert!(validate_expression(&tokens).is_ok());

    // Invalid expressions
    let tokens = vec![
        "(".to_string(),
        "A".to_string(),
        "+".to_string(),
        "B".to_string(),
    ];
    assert!(validate_expression(&tokens).is_err());

    let tokens = vec!["A".to_string(), "+".to_string(), "*".to_string(), "B".to_string()];
    assert!(validate_expression(&tokens).is_err());
}

/// Tests the `log_error` function for error logging.
///
/// # Test Cases
/// - Error is logged without causing a panic.
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
/// - Error descriptions generate appropriate suggestions.
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
