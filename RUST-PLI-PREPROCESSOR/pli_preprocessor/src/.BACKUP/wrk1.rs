#![allow(dead_code)] // Suppress warnings for unused functions in this module.
////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Tokenizer
// -----------------------------------------------------------------------------
// Description:
// This module provides functionality to tokenize lines of PL/I preprocessor 
// code. It splits input lines into meaningful tokens, including strings, 
// directives, operators, and special characters.
//
// Functions:
// - `tokenize_pli`: Splits a line into tokens.
// - `is_valid_preprocessor_directive`: Checks if the line starts with a valid
//   preprocessor directive.
// - `has_tokenizer_error`: Detects tokenizer errors (e.g., unclosed strings).
//
// Author: Jean-Pierre Sainfeld
// Assistant: ChatGPT
// Company: FirstLink Consulting Services (FLCS)
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

/// Tokenizes a single line of PL/I code.
/// 
/// This function splits the input string into meaningful tokens such as 
/// strings, directives, and operators.
/// 
/// # Arguments
/// - `text`: A reference to a string slice containing the line to tokenize.
///
/// # Returns
/// A `Vec<String>` containing the tokens extracted from the input line.


pub fn tokenize_pli(text: &str) -> Vec<String> {
    let mut tokens = Vec::new(); // Vector to hold tokens.
    let mut current_token = String::new(); // Temporary buffer for building tokens.
    let mut in_string = false; // Boolean flag to track if we're inside a string literal.

    let mut chars = text.chars().peekable(); // Create an iterator over characters with lookahead.

    while let Some(c) = chars.next() {
        if in_string {
            current_token.push(c); // Append character to the current token.
            if c == '\'' {
                // End of a string literal.
                in_string = false;
                tokens.push(current_token.clone()); // Save the token.
                current_token.clear(); // Reset for the next token.
            }
        } else if c == '\'' {
            // Start of a string literal.
            in_string = true;
            current_token.push(c);
        } else if c == '%' {
            // Start of a preprocessor directive.
            current_token.push(c);
            while let Some(&next_c) = chars.peek() {
                // Lookahead to include alphanumeric characters.
                if next_c.is_alphanumeric() {
                    current_token.push(next_c);
                    chars.next(); // Consume the character.
                } else {
                    break;
                }
            }
            tokens.push(current_token.to_uppercase()); // Save the directive token.
            current_token.clear();
        } else if c.is_whitespace() {
            // End of a token when whitespace is encountered.
            if !current_token.is_empty() {
                tokens.push(current_token.to_uppercase()); // Convert to uppercase
                current_token.clear();
            }
        } else if "!@#$%^&*()-+=[]{}|\\:;,.<>?/".contains(c) {
            // Handle special characters as individual tokens.
            if !current_token.is_empty() {
                tokens.push(current_token.to_uppercase()); // Convert to uppercase 
                current_token.clear();
            }
            tokens.push(c.to_string()); // Save the special character as a token.
        } else {
            // Build regular tokens.
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token.to_uppercase()); // Save the last token if any in uppercase.
    }

    tokens
}

/// Determines whether a line starts with a valid preprocessor directive.
///
/// # Arguments
/// - `tokens`: A reference to a vector of tokens.
///
/// # Returns
/// `true` if the first token matches a valid directive, otherwise `false`.
pub fn is_valid_preprocessor_directive(tokens: &[String]) -> bool {
    let valid_directives = ["%IF", "%DO", "%MACRO", "%END", "%ENDIF", "%INCLUDE", "%COMMENT"];
    if let Some(first_token) = tokens.get(0) {
        return valid_directives.contains(&first_token.as_str());
    }
    false
}

/// Checks for errors in tokenized lines (e.g., unclosed strings).
///
/// # Arguments
/// - `tokens`: A reference to a vector of tokens.
///
/// # Returns
/// `true` if a tokenizer error is found, otherwise `false`.
pub fn has_tokenizer_error(tokens: &[String]) -> bool {
    tokens.iter().any(|token| token.starts_with("'") && !token.ends_with("'"))
}

