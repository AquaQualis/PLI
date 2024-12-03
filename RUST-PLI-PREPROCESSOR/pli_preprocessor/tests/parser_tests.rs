////////////////////////////////////////////////////////////////////////////////
// TEST MODULE: Parser Tests
// -----------------------------------------------------------------------------
// Description:
// This module contains unit tests for the `parser` module. Each function tests
// specific functionality, ensuring correctness and robustness of the parsing logic.
//
// Tests cover:
// - Tokenization and categorization of single lines.
// - Extraction and validation of directives.
// - Multiline directive handling and error detection.
// - Parsing of control structures like DO, IF/THEN/ELSE, and SELECT.
// - Parsing of expressions, including operator precedence and associativity.
//
// -----------------------------------------------------------------------------
// FUNCTION INVENTORY:
// -----------------------------------------------------------------------------
// - test_parse_line: Tests single-line parsing functionality.
// - test_parse_source: Tests full-source parsing and directive extraction.
// - test_parse_statement: Tests single-statement parsing logic.
// - test_parse_control_structure: Tests control structure parsing and validation.
// - test_parse_expression: Tests expression parsing and operator precedence.
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
};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// UNIT TESTS
////////////////////////////////////////////////////////////////////////////////

/// TEST: test_parse_line
/// ----------------------------------------------------------------------------
/// Description:
/// Tests the functionality of `parse_line` to tokenize a single line of PL/I
/// source code into meaningful tokens.
/// ----------------------------------------------------------------------------
#[test]
fn test_parse_line() {
    // Test with a simple line
    let tokens = parse_line("DECLARE X FIXED;");
    assert_eq!(tokens, vec!["DECLARE", "X", "FIXED", ";"]);

    // Test with extra spaces
    let tokens = parse_line("   %INCLUDE   'file.pli';   ");
    assert_eq!(tokens, vec!["%INCLUDE", "'file.pli'", ";"]);

    // Test with empty line
    let tokens = parse_line("");
    assert!(tokens.is_empty());

    // Test with unexpected characters
    let tokens = parse_line("DECLARE @X;");
    assert_eq!(tokens, vec!["DECLARE", "@", "X", ";"]);
}

/// TEST: test_parse_source
/// ----------------------------------------------------------------------------
/// Description:
/// Validates the `parse_source` function for tokenizing and extracting
/// directives from full PL/I source code.
/// ----------------------------------------------------------------------------
#[test]
fn test_parse_source() {
    let source = "DECLARE X FIXED;\n%INCLUDE 'example.pli';";
    let mut directives = HashMap::new();

    let result = parse_source(source, &mut directives).unwrap();

    // Validate tokenized lines
    assert_eq!(result, vec![vec!["DECLARE", "X", "FIXED", ";"],]);
    assert!(directives.contains_key("%INCLUDE 'example.pli';"));
    assert_eq!(
        directives["%INCLUDE 'example.pli';"],
        vec!["%INCLUDE", "'example.pli'", ";"]
    );

    // Test with empty source
    let empty_source = "";
    let mut empty_directives = HashMap::new();
    let empty_result = parse_source(empty_source, &mut empty_directives).unwrap();
    assert!(empty_result.is_empty());
    assert!(empty_directives.is_empty());
}

/// TEST: test_parse_statement
/// ----------------------------------------------------------------------------
/// Description:
/// Tests `parse_statement` for accurate tokenization of single-line PL/I
/// statements, including handling of multi-part identifiers.
/// ----------------------------------------------------------------------------
#[test]
fn test_parse_statement() {
    // Simple statement
    let tokens = parse_statement("UNKNOWN_STATEMENT;");
    assert_eq!(tokens, vec!["UNKNOWN_STATEMENT", ";"]);

    // Multi-part statement
    let tokens = parse_statement("MULTI_PART_STATEMENT;");
    assert_eq!(tokens, vec!["MULTI_PART_STATEMENT", ";"]);

    // Empty statement
    let tokens = parse_statement("");
    assert!(tokens.is_empty());
}

/// TEST: test_parse_control_structure
/// ----------------------------------------------------------------------------
/// Description:
/// Tests the `parse_control_structure` function for handling and validating
/// control structures like DO/END, including nested constructs.
/// ----------------------------------------------------------------------------
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

/// TEST: test_parse_expression
/// ----------------------------------------------------------------------------
/// Description:
/// Tests the `parse_expression` function for evaluating expressions with
/// operator precedence and associativity.
/// ----------------------------------------------------------------------------
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

////////////////////////////////////////////////////////////////////////////////
// END OF TEST MODULE
////////////////////////////////////////////////////////////////////////////////
