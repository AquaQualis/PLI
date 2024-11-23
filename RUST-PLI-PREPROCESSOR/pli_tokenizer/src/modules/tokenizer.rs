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

    // Initialize some useful variables
    let mut tokens = Vec::new(); // Vector to hold tokens.
    let mut current_token = String::new(); // Temporary buffer for building tokens.
    let mut in_string = false; // Boolean flag to track if we're inside a string literal.

    let mut chars = text.chars().peekable(); // Create an iterator over characters with lookahead.

    // Process each character 
    while let Some(c) = chars.next() {

        // Handle string literals enclosed in single quotes (e.g., 'text').
        // Collects all characters until the closing quote and saves it as a single token.
        // Detects when you're inside a string literal (e.g., 'hello world').
        // Appends all characters, including quotes, to current_token.
        //Once the closing ' is found, it saves the string as a token.
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

        // Handle Preprocessor Directives
        // Starts a directive when % is encountered (e.g., %IF, %THEN).
        // Collects alphanumeric characters following % to form the full directive token.
        // Converts it to uppercase (e.g., %if → %IF) before saving it.
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

        // Handle Whitespace
        // Ends the current token when whitespace is encountered.
        // Converts the current token to uppercase before saving.
        } else if c.is_whitespace() {
            // End of a token when whitespace is encountered.
            if !current_token.is_empty() {
                tokens.push(current_token.to_uppercase()); // Convert to uppercase.
                current_token.clear();
            }

        // Handle Special Characters
        // Treats special characters (!@#$%^&*, etc.) as tokens.
        // Combines a special character (e.g., @) with alphanumeric characters if present (e.g., @INVALID_CHAR).

        } else if "!@#$%^&*()-+=[]{}|\\:;,.<>?/".contains(c) {
            // Handle special characters as standalone tokens or part of combined tokens.
            if !current_token.is_empty() {
                tokens.push(current_token.to_uppercase()); // Save the previous token in uppercase.
                current_token.clear();
            }

            // Check if the special character should be appended to the last token.
            if ".,".contains(c) && !tokens.is_empty() && tokens.last().unwrap().chars().all(char::is_alphabetic) {
                // Append punctuation to the previous token if it’s plain text.
                let last_token = tokens.pop().unwrap();
                tokens.push(format!("{}{}", last_token, c).to_uppercase());
            } else {
                current_token.push(c); // Start with the special character.
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '_' {
                        current_token.push(next_c); // Combine alphanumeric characters.
                        chars.next(); // Consume the character.
                    } else {
                        break;
                    }
                }
                tokens.push(current_token.to_uppercase()); // Save the combined token.
                current_token.clear();
            }

        // Handle regular token
        // Builds "regular" tokens by appending characters.
        } else {
            // Build regular tokens.
            current_token.push(c);
        }
    }

    // Handle last token
    // Ensures the last token (if any) is saved after the loop ends.
    if !current_token.is_empty() {
        tokens.push(current_token.to_uppercase()); // Save the last token if any in uppercase.
    }

    println!("Tokens for '{}': {:?}", text, tokens); // Log the tokens generated.
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

