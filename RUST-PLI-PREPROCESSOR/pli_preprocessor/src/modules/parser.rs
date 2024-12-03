#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Parser
// -----------------------------------------------------------------------------
// Description:
// This module handles parsing of PL/I source code for tokenization, syntax
// validation, and expression parsing. It processes control structures,
// validates syntax, and provides a foundation for higher-level constructs like
// AST generation.
//
// Features:
// - Parsing control structures (e.g., DO, IF/THEN/ELSE, SELECT).
// - Parsing and evaluating expressions with operator precedence.
// - Handling nested constructs using a stack or recursion.
// - Syntax validation for matched constructs and expressions.
//
// -----------------------------------------------------------------------------
// FUNCTION INVENTORY:
// -----------------------------------------------------------------------------
// - parse_line: Tokenizes and categorizes a single line of PL/I source code.
// - parse_statement: Processes single-line PL/I statements.
// - parse_source: Processes the entire PL/I source and extracts directives.
// - parse_control_structure: Parses and validates control structures.
// - parse_expression: Parses and validates expressions with operator precedence.
// - handle_multiline: Handles multiline directives in the source.
// - validate_syntax: Checks for syntax errors and consistency.
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

/// Parses an expression, respecting operator precedence.
///
/// # Arguments
/// - `tokens`: A `&[String]` slice representing the tokens of the expression.
///
/// # Returns
/// - `Result<Vec<String>, String>`: Returns a vector of tokens in reverse Polish
///   notation (RPN) for evaluation, or an error message if parsing fails.
///
/// # Example
/// ```rust
/// let tokens = vec!["(", "A", "+", "B", ")", "*", "C"];
/// let rpn = parse_expression(&tokens).unwrap();
/// assert_eq!(rpn, vec!["A", "B", "+", "C", "*"]);
/// ```
pub fn parse_expression(tokens: &[String]) -> Result<Vec<String>, String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    // Operator precedence table
    let precedence: HashMap<&str, u8> = HashMap::from([
        ("*", 3),
        ("/", 3),
        ("+", 2),
        ("-", 2),
        ("AND", 1),
        ("OR", 1),
    ]);

    for token in tokens {
        match token.as_str() {
            // If token is an operand, add it to the output
            t if t.chars().all(char::is_alphanumeric) => output.push(t.to_string()),
            // If token is an operator
            t if precedence.contains_key(t) => {
                while let Some(op) = operators.last() {
                    if precedence.get(op.as_str()) >= precedence.get(t) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(t.to_string());
            }
            // Handle parentheses
            "(" => operators.push(token.to_string()),
            ")" => {
                while let Some(op) = operators.pop() {
                    if op == "(" {
                        break;
                    }
                    output.push(op);
                }
            }
            // Invalid token
            _ => return Err(format!("Invalid token in expression: {}", token)),
        }
    }

    // Pop remaining operators to the output
    while let Some(op) = operators.pop() {
        if op == "(" || op == ")" {
            return Err("Mismatched parentheses in expression.".to_string());
        }
        output.push(op);
    }

    Ok(output)
}



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
        match ch {
            '\'' => {
                if inside_quotes {
                    buffer.push(ch);
                    tokens.push(buffer.clone());
                    buffer.clear();
                } else {
                    if !buffer.is_empty() {
                        tokens.push(buffer.clone());
                        buffer.clear();
                    }
                    buffer.push(ch);
                }
                inside_quotes = !inside_quotes;
            }
            _ if inside_quotes => buffer.push(ch),
            ch if ch.is_whitespace() => {
                if !buffer.is_empty() {
                    tokens.push(buffer.clone());
                    buffer.clear();
                }
            }
            '%' => {
                if !buffer.is_empty() {
                    tokens.push(buffer.clone());
                    buffer.clear();
                }
                buffer.push(ch);
            }
            ch if ch.is_alphanumeric() || ch == '_' => buffer.push(ch),
            ch => {
                if !buffer.is_empty() {
                    tokens.push(buffer.clone());
                    buffer.clear();
                }
                tokens.push(ch.to_string());
            }
        }
    }

    if !buffer.is_empty() {
        tokens.push(buffer);
    }

    tokens
}

/// Parses a single PL/I statement into meaningful tokens.
///
/// # Arguments
/// - `statement`: A `&str` containing the statement.
///
/// # Returns
/// - `Vec<String>`: Returns a vector of tokens representing the statement.
///
/// # Example
/// ```rust
/// let tokens = parse_statement("UNKNOWN_STATEMENT;");
/// assert_eq!(tokens, vec!["UNKNOWN_STATEMENT", ";"]);
/// ```
pub fn parse_statement(statement: &str) -> Vec<String> {
    parse_line(statement)
        .iter()
        .fold(Vec::new(), |mut acc, token| {
            if let Some(last) = acc.last_mut() {
                if token.starts_with('_') || last.ends_with('_') {
                    last.push_str(token);
                    return acc;
                }
            }
            acc.push(token.clone());
            acc
        })
}

/// Parses control structures (e.g., DO/END) and validates their syntax.
///
/// # Arguments
/// - `tokens`: A `Vec<String>` representing tokens of a control structure.
///
/// # Returns
/// - `Result<(), String>`: Returns `Ok(())` if the structure is valid, or an error message if invalid.
pub fn parse_control_structure(tokens: Vec<String>) -> Result<(), String> {
    let mut stack = Vec::new();

    for token in tokens {
        match token.as_str() {
            "DO" => stack.push(token.clone()), // Push owned value into the stack
            "END" => {
                if stack.pop() != Some("DO".to_string()) {
                    return Err("Unmatched END".to_string());
                }
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        Err("Unclosed DO".to_string())
    } else {
        Ok(())
    }
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
