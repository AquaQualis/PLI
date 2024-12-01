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
//
// -----------------------------------------------------------------------------
// FUNCTION INVENTORY:
// -----------------------------------------------------------------------------
// - test_parse_line: Tests single-line parsing functionality.
// - test_parse_source: Tests full-source parsing and directive extraction.
// - test_parse_statement: Tests single-statement parsing logic.
// - test_parse_control_structure: Tests control structure parsing and validation.
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

use pli_preprocessor::modules::parser::{parse_line, parse_source, parse_statement, parse_control_structure};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// UNIT TESTS
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_parse_line() {
    let tokens = parse_line("DECLARE X FIXED;");
    assert_eq!(tokens, vec!["DECLARE", "X", "FIXED", ";"]);

    let tokens = parse_line("   %INCLUDE   'file.pli';   ");
    assert_eq!(tokens, vec!["%INCLUDE", "'file.pli'", ";"]);
}

#[test]
fn test_parse_source() {
    let source = "DECLARE X FIXED;\n%INCLUDE 'example.pli';";
    let mut directives = HashMap::new();

    let result = parse_source(source, &mut directives).unwrap();

    assert_eq!(result, vec![vec!["DECLARE", "X", "FIXED", ";"],]);
    assert!(directives.contains_key("%INCLUDE 'example.pli';"));
    assert_eq!(
        directives["%INCLUDE 'example.pli';"],
        vec!["%INCLUDE", "'example.pli'", ";"]
    );
}

#[test]
fn test_parse_statement() {
    let tokens = parse_statement("UNKNOWN_STATEMENT;");
    assert_eq!(tokens, vec!["UNKNOWN_STATEMENT", ";"]);

    let tokens = parse_statement("MULTI_PART_STATEMENT;");
    assert_eq!(tokens, vec!["MULTI_PART_STATEMENT", ";"]);
}

#[test]
fn test_parse_control_structure() {
    // Valid DO/END structure
    let tokens = vec!["DO".to_string(), "I".to_string(), "=".to_string(), "1".to_string(), "TO".to_string(), "10".to_string(), ";".to_string(), "END".to_string(), ";".to_string()];
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
    let tokens = vec!["DO".to_string(), "I".to_string(), "=".to_string(), "1".to_string(), "TO".to_string(), "10".to_string(), ";".to_string()];
    assert!(parse_control_structure(tokens).is_err());

    // Unmatched END
    let tokens = vec!["END".to_string(), ";".to_string()];
    assert!(parse_control_structure(tokens).is_err());
}
