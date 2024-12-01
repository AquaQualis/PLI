#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Parser
// -----------------------------------------------------------------------------
// Description:
// This module handles parsing of PL/I source code for tokenization and syntax
// validation. It converts source code into meaningful tokens, validates syntax,
// and provides a foundation for higher-level constructs like AST generation.
//
// Features:
// - Parsing PL/I source code into structured tokens.
// - Identification and categorization of directives, statements, and expressions.
// - Handling multiline directives and concatenated strings.
// - Support for escape sequences and nested constructs.
// - Validation of syntax and error reporting.
//
// -----------------------------------------------------------------------------
// FUNCTION INVENTORY:
// -----------------------------------------------------------------------------
// - parse_line: Tokenizes and categorizes a single line of PL/I source code.
// - parse_source: Processes the entire PL/I source and extracts directives.
// - handle_multiline: Handles multiline directives in the source.
// - validate_syntax: Checks for basic syntax errors and consistency.
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

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

/// Parses a single line of PL/I source code into tokens.
///
/// # Arguments
/// - `line`: A `&str` representing the source code line.
///
/// # Returns
/// - `Vec<String>`: Returns a vector of tokens extracted from the line.
///
/// # Example
/// ```rust
/// let tokens = parse_line("DECLARE X FIXED;");
/// assert_eq!(tokens, vec!["DECLARE", "X", "FIXED", ";"]);
/// ```
pub fn parse_line(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut inside_quotes = false;

    for ch in line.chars() {
        if ch == '\'' {
            if inside_quotes {
                buffer.push(ch); // Add the closing quote
                tokens.push(buffer.clone());
                buffer.clear();
            } else {
                if !buffer.is_empty() {
                    tokens.push(buffer.clone());
                    buffer.clear();
                }
                buffer.push(ch); // Start a quoted token
            }
            inside_quotes = !inside_quotes;
        } else if inside_quotes {
            buffer.push(ch);
        } else if ch.is_whitespace() {
            if !buffer.is_empty() {
                tokens.push(buffer.clone());
                buffer.clear();
            }
        } else if ch == '%' && buffer.is_empty() {
            buffer.push(ch); // Start a directive token
        } else if buffer.starts_with('%') {
            if ch.is_alphanumeric() {
                buffer.push(ch); // Continue building the directive
            } else {
                tokens.push(buffer.trim().to_string());
                buffer.clear();
                if !ch.is_whitespace() {
                    tokens.push(ch.to_string()); // Add punctuation as a separate token
                }
            }
        } else if ch.is_ascii_punctuation() {
            if !buffer.is_empty() {
                tokens.push(buffer.clone());
                buffer.clear();
            }
            tokens.push(ch.to_string());
        } else {
            buffer.push(ch);
        }
    }

    if !buffer.is_empty() {
        tokens.push(buffer.clone());
    }

    tokens
}

/// Parses the entire PL/I source code into structured tokens.
///
/// # Arguments
/// - `source`: A `&str` containing the full source code.
/// - `directives`: A `&mut HashMap<String, Vec<String>>` for storing parsed directives.
///
/// # Returns
/// - `Result<Vec<Vec<String>>, String>`: Returns a vector of tokenized lines,
///   or an error message if parsing fails.
///
/// # Example
/// ```rust
/// let mut directives = HashMap::new();
/// let result = parse_source("DECLARE X FIXED;\n%INCLUDE 'example.pli';", &mut directives);
/// assert!(result.is_ok());
/// ```
pub fn parse_source(
    source: &str,
    directives: &mut HashMap<String, Vec<String>>,
) -> Result<Vec<Vec<String>>, String> {
    let mut tokenized_lines = Vec::new();

    for line in source.lines() {
        if line.trim().starts_with('%') {
            directives.insert(line.to_string(), parse_line(line));
        } else {
            tokenized_lines.push(parse_line(line));
        }
    }

    Ok(tokenized_lines)
}
