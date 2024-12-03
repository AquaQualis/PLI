#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Parser
// -----------------------------------------------------------------------------
// Description:
// This module handles parsing of PL/I source code for tokenization, syntax
// validation, control structures, expression parsing, and error handling. It
// processes control structures, validates syntax, and provides a foundation for
// higher-level constructs like Abstract Syntax Tree (AST) generation.
//
// Features:
// - Parsing control structures (e.g., DO, IF/THEN/ELSE, SELECT).
// - Parsing and evaluating expressions with operator precedence.
// - Handling nested constructs using a stack or recursion.
// - Syntax validation and error recovery mechanisms.
// - Support for multiline directives.
//
// -----------------------------------------------------------------------------
// FUNCTION INVENTORY:
// -----------------------------------------------------------------------------
// - parse_line: Tokenizes and categorizes a single line of PL/I source code.
// - parse_statement: Processes single-line PL/I statements.
// - parse_source: Processes the entire PL/I source and extracts directives.
// - parse_control_structure: Parses and validates control structures.
// - parse_expression: Parses and validates expressions with operator precedence.
// - validate_expression: Validates expressions and ensures syntactic correctness.
// - recover_from_error: Attempts to recover after detecting a parsing error.
// - log_error: Logs error details for review.
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
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

/// Represents an error encountered during parsing.
#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub token: Option<String>,
    pub description: String,
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

/// Parses a single line of PL/I source code into tokens.
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
pub fn parse_control_structure(tokens: Vec<String>) -> Result<(), String> {
    let mut stack = Vec::new();

    for token in tokens {
        match token.as_str() {
            "DO" => stack.push(token.clone()),
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

/// Parses an expression, respecting operator precedence.
pub fn parse_expression(tokens: &[String]) -> Result<Vec<String>, String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

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
            t if t.chars().all(char::is_alphanumeric) => output.push(t.to_string()),
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
            "(" => operators.push(token.to_string()),
            ")" => {
                while let Some(op) = operators.pop() {
                    if op == "(" {
                        break;
                    }
                    output.push(op);
                }
            }
            _ => return Err(format!("Invalid token in expression: {}", token)),
        }
    }

    while let Some(op) = operators.pop() {
        if op == "(" || op == ")" {
            return Err("Mismatched parentheses in expression.".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

/// Validates an expression for syntax correctness.
pub fn validate_expression(tokens: &[String]) -> Result<(), String> {
    let mut parentheses_stack: Vec<char> = Vec::new();
    let valid_operators = ["+", "-", "*", "/", "AND", "OR"];
    let mut last_token: Option<&str> = None;

    for token in tokens {
        match token.as_str() {
            "(" => parentheses_stack.push('('),
            ")" => {
                if parentheses_stack.pop().is_none() {
                    return Err("Unmatched closing parenthesis".to_string());
                }
            }
            t if valid_operators.contains(&t) => {
                if let Some(last) = last_token {
                    if valid_operators.contains(&last) || last == "(" {
                        return Err(format!("Invalid operator placement: '{}'", t));
                    }
                }
            }
            t if t.chars().all(char::is_alphanumeric) => { /* Valid operand */ }
            _ => return Err(format!("Invalid token in expression: '{}'", token)),
        }
        last_token = Some(token.as_str());
    }

    if !parentheses_stack.is_empty() {
        return Err("Unmatched opening parenthesis".to_string());
    }

    Ok(())
}

/// Parses the entire PL/I source code into structured tokens.
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

/// Logs a parsing error for debugging purposes.
pub fn log_error(error: &ParseError) {
    println!(
        "Parse Error at line {}: {} - {:?}",
        error.line, error.description, error.token
    );
}

/// Attempts to recover from a parsing error.
pub fn recover_from_error(error: &ParseError) -> Option<String> {
    match error.description.as_str() {
        "Unmatched closing parenthesis" => Some("Add the missing opening parenthesis.".to_string()),
        "Unmatched opening parenthesis" => Some("Add the missing closing parenthesis.".to_string()),
        "Invalid operator placement" => Some("Check the operator placement in the expression.".to_string()),
        _ => None,
    }
}
