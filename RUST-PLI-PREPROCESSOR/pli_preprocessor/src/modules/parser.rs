#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Parser
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This module handles parsing of PL/I source code for tokenization and syntax
// validation. It splits the source code into meaningful tokens and identifies
// directives, statements, and expressions.
//
// FUNCTIONALITY:
// - Parses raw PL/I source code into structured tokens.
// - Distinguishes between directives, statements, and expressions.
// - Handles multiline directives and concatenated strings.
// - Ensures proper handling of escape sequences.
//
// USAGE:
// - Use `parse_line` to tokenize and categorize a single line of code.
// - Extend `parse_source` for processing entire files.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
// VERSION: 1.0.2
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

    println!("Parsing line: {:?}", line); // Debug: Show the input line

    for ch in line.chars() {
        println!("Processing character: {:?}", ch); // Debug: Show each character

        if ch == '\'' {
            println!("Quote encountered. Inside quotes: {}", inside_quotes); // Debug: Quote state
            if inside_quotes {
                buffer.push(ch); // Add the closing quote
                tokens.push(buffer.clone());
                println!("Token added (quoted): {:?}", buffer); // Debug: Quoted token
                buffer.clear();
            } else {
                if !buffer.is_empty() {
                    tokens.push(buffer.clone());
                    println!("Token added (before quote): {:?}", buffer); // Debug: Token before quote
                    buffer.clear();
                }
                buffer.push(ch); // Start a new quoted token
            }
            inside_quotes = !inside_quotes;
        } else if inside_quotes {
            buffer.push(ch);
        } else if ch.is_whitespace() {
            println!("Whitespace encountered. Current buffer: {:?}", buffer); // Debug: Whitespace
            if !buffer.is_empty() {
                tokens.push(buffer.clone());
                println!("Token added (whitespace): {:?}", buffer); // Debug: Token after whitespace
                buffer.clear();
            }
        } else if ch == '%' && buffer.is_empty() {
            buffer.push(ch); // Start a directive token
        } else if buffer.starts_with('%') {
            buffer.push(ch);
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                tokens.push(buffer.trim().to_string());
                println!("Token added (directive): {:?}", buffer.trim()); // Debug: Directive token
                buffer.clear();
            }
        } else if ch.is_ascii_punctuation() {
            println!("Punctuation encountered: {:?}", ch); // Debug: Punctuation
            if !buffer.is_empty() {
                tokens.push(buffer.clone());
                println!("Token added (before punctuation): {:?}", buffer); // Debug: Token before punctuation
                buffer.clear();
            }
            tokens.push(ch.to_string());
            println!("Token added (punctuation): {:?}", ch); // Debug: Punctuation token
        } else {
            buffer.push(ch);
        }
    }

    if !buffer.is_empty() {
        println!("Final token added: {:?}", buffer); // Debug: Final token
        tokens.push(buffer.clone());
        buffer.clear(); // Clear the buffer
    }

    println!("Tokens generated: {:?}", tokens); // Debug: Final token list
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
            // Capture directives separately
            directives.insert(line.to_string(), parse_line(line));
        } else {
            tokenized_lines.push(parse_line(line));
        }
    }

    Ok(tokenized_lines)
}
