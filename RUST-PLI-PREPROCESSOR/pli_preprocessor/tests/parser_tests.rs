////////////////////////////////////////////////////////////////////////////////
// TESTS FOR MODULE: Parser
// ----------------------------------------------------------------------------
// This file contains unit tests for the `parser` module. Each function tests
// specific functionality, ensuring correctness and robustness.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
// VERSION: 1.0.2
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
