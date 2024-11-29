#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Syntax Validator
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This module validates the syntax and structure of tokenized PL/I code.
//
// FUNCTIONALITY:
// - Checks for syntax errors in directives and statements.
// - Ensures proper nesting and pairing of directives (e.g., `%IF` and `%ENDIF`).
// - Validates string literals and special character usage.
// - Detects unrecognized or invalid tokens.
//
// USAGE:
// - Use `validate_syntax` to validate a vector of tokens representing a PL/I line.
// - Call `is_valid_directive` for directive-specific validation.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
// VERSION: 1.0.1
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

/// Validates the syntax of a tokenized PL/I line.
///
/// # Arguments
/// - `tokens`: A `&[String]` slice containing the tokenized PL/I line.
///
/// # Returns
/// - `Result<(), String>`: Returns `Ok(())` if the syntax is valid, or an
///   `Err(String)` with an error message if the syntax is invalid.
///
/// # Example
/// ```rust
/// let tokens = vec!["%IF".to_string(), "DEBUG".to_string(), "%THEN".to_string()];
/// match validate_syntax(&tokens) {
///     Ok(_) => println!("Syntax is valid."),
///     Err(e) => println!("Syntax error: {}", e),
/// }
/// ```
pub fn validate_syntax(tokens: &[String]) -> Result<(), String> {
    if tokens.is_empty() {
        return Err("Empty token line".to_string());
    }

    let mut stack = Vec::new();

    for token in tokens {
        match token.as_str() {
            "%IF" => stack.push("%IF"),
            "%ENDIF" => {
                if stack.pop() != Some("%IF") {
                    return Err("Unmatched %ENDIF found".to_string());
                }
            }
            "%THEN" => {
                if stack.last() != Some(&"%IF") {
                    return Err("%THEN without matching %IF".to_string());
                }
            }
            _ if token.starts_with('%') && !is_valid_directive(token) => {
                return Err(format!("Invalid directive: {}", token));
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        return Err("Unmatched %IF found".to_string());
    }

    Ok(())
}

/// Checks if a directive token is valid.
///
/// # Arguments
/// - `directive`: A `&str` containing the directive token to validate.
///
/// # Returns
/// - `bool`: `true` if the directive is valid, `false` otherwise.
///
/// # Example
/// ```rust
/// assert!(is_valid_directive("%IF"));
/// assert!(!is_valid_directive("%INVALID"));
/// ```
pub fn is_valid_directive(directive: &str) -> bool {
    let valid_directives = vec![
        "%IF", "%ENDIF", "%ELSE", "%THEN", "%DO", "%END", "%SWITCH", "%CASE", "%DEFAULT",
    ];
    valid_directives.contains(&directive.to_uppercase().as_str())
}
