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
// - Comprehensive edge-case scenarios.
//
// -----------------------------------------------------------------------------
// FUNCTION INVENTORY:
// -----------------------------------------------------------------------------
// - test_parse_line: Tests single-line parsing functionality.
// - test_parse_source: Tests full-source parsing and directive extraction.
// - test_multiline_directives: Tests handling of multiline directives.
// - test_error_handling: Tests syntax error detection in various scenarios.
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

use pli_tokenizer::modules::parser::{parse_line, parse_source};
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
 
