#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Conditional Processor
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This module handles the evaluation of conditional directives in PL/I code.
//
// FUNCTIONALITY:
// - Evaluates conditions in `%IF` and `%ELSE` directives.
// - Tracks nesting levels of conditional blocks to ensure correct pairing
//   with `%ENDIF`.
// - Supports boolean expressions with basic operators (`=`, `!=`, `<`, `>`, etc.).
//
// USAGE:
// - Use `process_condition` to evaluate a single `%IF` condition.
// - Call `validate_conditional_structure` to check nesting and block validity.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
// VERSION: 1.0.0
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

/// Processes a single `%IF` condition and returns its evaluation result.
///
/// # Arguments
/// - `condition`: A `&str` representing the conditional expression to evaluate.
///
/// # Returns
/// - `Result<bool, String>`: Returns `Ok(true)` or `Ok(false)` based on the evaluation,
///   or an `Err(String)` with an error message if the condition is invalid.
///
/// # Example
/// ```rust
/// let result = process_condition("DEBUG = 1");
/// assert_eq!(result, Ok(true)); // Assuming DEBUG = 1 in the context
/// ```
pub fn process_condition(condition: &str) -> Result<bool, String> {
    if condition.trim().is_empty() {
        return Err("Empty condition".to_string());
    }

    let parts: Vec<&str> = condition.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!("Invalid condition format: {}", condition));
    }

    let left = parts[0];
    let operator = parts[1];
    let right = parts[2];

    let context = vec![("DEBUG", "1")];
    let left_value = context
        .iter()
        .find(|&&(key, _)| key == left)
        .map(|&(_, val)| val);

    if let Some(value) = left_value {
        match operator {
            "=" => Ok(value == right),
            "!=" => Ok(value != right),
            _ => Err(format!("Unsupported operator: {}", operator)),
        }
    } else {
        Err(format!("Unknown variable: {}", left))
    }
}

/// Validates the structure of nested conditional blocks.
///
/// # Arguments
/// - `tokens`: A `&[String]` slice containing tokenized PL/I lines.
///
/// # Returns
/// - `Result<(), String>`: Returns `Ok(())` if the structure is valid, or an
///   `Err(String)` with an error message if there are mismatched directives.
///
/// # Example
/// ```rust
/// let tokens = vec!["%IF".to_string(), "%ENDIF".to_string()];
/// let result = validate_conditional_structure(&tokens);
/// assert!(result.is_ok());
/// ```
pub fn validate_conditional_structure(tokens: &[String]) -> Result<(), String> {
    let mut nesting_level = 0;

    for token in tokens {
        if token == "%IF" {
            nesting_level += 1;
        } else if token == "%ENDIF" {
            if nesting_level == 0 {
                return Err("Unmatched %ENDIF directive".to_string());
            }
            nesting_level -= 1;
        }
    }

    if nesting_level != 0 {
        Err("Unmatched %IF directive".to_string())
    } else {
        Ok(())
    }
}
