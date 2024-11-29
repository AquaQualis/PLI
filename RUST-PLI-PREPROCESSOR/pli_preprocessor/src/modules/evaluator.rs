#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Expression Evaluator
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This module handles the evaluation of mathematical and logical expressions
// in the PL/I preprocessor. It supports operators like `+`, `-`, `*`, `/`, `AND`, `OR`, etc.
//
// FUNCTIONALITY:
// - Parses and evaluates expressions used in PL/I directives.
// - Supports precedence and associativity for operators.
// - Handles variables with values from a predefined context.
// - Converts infix expressions to postfix notation for correct evaluation.
//
// USAGE:
// - Use `evaluate_expression` to compute the result of an expression.
// - Extend the `evaluate_operator` function to support more operators.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
// VERSION: 2.0.1
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

/// Evaluates a mathematical or logical expression and returns the result.
///
/// # Arguments
/// - `expression`: A `&str` containing the expression to evaluate (e.g., `"3 + 5"`).
///
/// # Returns
/// - `Result<i32, String>`: Returns `Ok(result)` with the computed value, or an
///   `Err(String)` with an error message if the expression is invalid.
///
/// # Example
/// ```rust
/// let result = evaluate_expression("3 + 5");
/// assert_eq!(result, Ok(8));
/// ```
pub fn evaluate_expression(expression: &str) -> Result<i32, String> {
    if expression.trim().is_empty() {
        return Err("Expression is empty".to_string());
    }

    let tokens = tokenize_expression(expression)?;
    parse_and_evaluate(&tokens)
}

/// Tokenizes an expression into a list of operators and operands.
///
/// # Arguments
/// - `expression`: A `&str` containing the expression to tokenize.
///
/// # Returns
/// - `Result<Vec<String>, String>`: Returns a vector of tokens or an error message.
///
/// # Example
/// ```rust
/// let tokens = tokenize_expression("3 + 5");
/// assert_eq!(tokens, Ok(vec!["3", "+", "5"]));
/// ```
pub fn tokenize_expression(expression: &str) -> Result<Vec<String>, String> {
    if expression.trim().is_empty() {
        return Err("Expression is empty".to_string());
    }

    let tokens: Vec<String> = expression
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    Ok(tokens)
}

/// Parses and evaluates a list of tokens.
///
/// # Arguments
/// - `tokens`: A `&[String]` slice containing the tokenized expression.
///
/// # Returns
/// - `Result<i32, String>`: Returns the computed result or an error message.
///
/// # Example
/// ```rust
/// let tokens = vec!["3".to_string(), "+".to_string(), "5".to_string()];
/// let result = parse_and_evaluate(&tokens);
/// assert_eq!(result, Ok(8));
/// ```
pub fn parse_and_evaluate(tokens: &[String]) -> Result<i32, String> {
    if tokens.is_empty() {
        return Err("No tokens to evaluate".to_string());
    }

    // Convert infix expression to postfix (Reverse Polish Notation)
    let postfix_tokens = infix_to_postfix(tokens)?;
    println!("Postfix Tokens: {:?}", postfix_tokens); // Debug: Postfix representation

    let mut stack: Vec<i32> = Vec::new();

    // Evaluate the postfix expression
    for token in postfix_tokens {
        if let Ok(num) = token.parse::<i32>() {
            // If the token is a number, push it onto the stack
            stack.push(num);
        } else {
            // If the token is an operator, ensure there are enough operands
            if stack.len() < 2 {
                println!(
                    "Malformed Expression: Stack: {:?}, Operator: {}",
                    stack, token
                ); // Debug: Stack state
                return Err("Malformed expression".to_string());
            }

            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            println!(
                "Stack Before: {:?}, Operator: {}, Operands: ({}, {})",
                stack, token, a, b
            ); // Debug: Before operation

            // Perform the operation and push the result onto the stack
            let result = evaluate_operator(a, b, &token)?;
            stack.push(result);

            println!("Stack After: {:?}", stack); // Debug: After operation
        }
    }

    if stack.len() != 1 {
        println!("Final Stack State: {:?}", stack); // Debug: Final stack state
        return Err("Malformed expression".to_string());
    }

    Ok(stack[0])
}

/// Converts an infix expression to postfix (RPN).
///
/// # Arguments
/// - `tokens`: A slice of infix tokens.
///
/// # Returns
/// - `Result<Vec<String>, String>`: Returns a vector of postfix tokens or an error.
///
/// # Example
/// ```rust
/// let tokens = vec!["3".to_string(), "+".to_string(), "5".to_string()];
/// let result = infix_to_postfix(&tokens);
/// assert_eq!(result, Ok(vec!["3".to_string(), "5".to_string(), "+".to_string()]));
/// ```
fn infix_to_postfix(tokens: &[String]) -> Result<Vec<String>, String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    let precedence = |op: &str| match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    };

    let mut expect_operand = true;

    for token in tokens {
        if let Ok(_) = token.parse::<i32>() {
            output.push(token.clone());
            expect_operand = false;
        } else if ["+", "-", "*", "/"].contains(&token.as_str()) {
            if expect_operand {
                return Err(format!("Operator '{}' without operand", token));
            }
            while let Some(op) = operators.last() {
                if precedence(op) >= precedence(token) {
                    output.push(operators.pop().unwrap());
                } else {
                    break;
                }
            }
            operators.push(token.clone());
            expect_operand = true;
        } else {
            return Err(format!("Unsupported token: {}", token));
        }
    }

    if expect_operand {
        return Err("Expression ends with operator".to_string());
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    Ok(output)
}

/// Evaluates a binary operation.
///
/// # Arguments
/// - `a`: The left operand.
/// - `b`: The right operand.
/// - `operator`: A `&str` representing the operator (e.g., `+`, `-`, `*`, `/`).
///
/// # Returns
/// - `Result<i32, String>`: Returns the result of the operation or an error message.
///
/// # Example
/// ```rust
/// let result = evaluate_operator(3, 5, "+");
/// assert_eq!(result, Ok(8));
/// ```
pub fn evaluate_operator(a: i32, b: i32, operator: &str) -> Result<i32, String> {
    match operator {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Unsupported operator: {}", operator)),
    }
}
