////////////////////////////////////////////////////////////////////////////////
// PL/I Preprocessor Tokenizer
// -----------------------------------------------------------------------------
// Author: Jean-Pierre Sainfeld
// Assitant: ChatGpt
// Company: FirstLink Consulting Services (FLCS)
// Date: 11/17/2024
// -----------------------------------------------------------------------------
// Description:
// This program implements a tokenizer for a simplified PL/I preprocessor.
// It processes files containing PL/I preprocessor directives and normal
// PL/I statements, tokenizes each line, and logs the results, including
// pass/fail status, to a log file.
//
// Features:
// - Tokenizes strings, directives, operators, and special characters.
// - Validates file extensions to accept only `.pp` and `.pli` files.
// - Logs tokenized lines and summary results.
// - Supports basic PL/I preprocessor directives like `%IF`, `%THEN`, `%ELSE`.
//
// Purpose:
// This project is a learning exercise to explore the Rust programming
// language while implementing a functional PL/I preprocessor tokenizer.
//
// Usage:
// Run the program with a file as input:
// $ cargo run <input_file>
//
// The results will be written to a log file in `tests/output/pli_tokenizer.log`.
//
// Company Mission:
// At FirstLink Consulting Services (FLCS), we specialize in delivering
// innovative solutions for complex software challenges.
//
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

use std::env; // Import the `env` module to handle command-line arguments.
use std::fs::File; // Import the `File` module to perform file I/O operations.
use std::io::{self, BufRead, Write}; // Import `io` for reading/writing and buffer handling.
use std::path::Path; // Import `Path` to handle file path manipulations.

/// Tokenizes a single line of PL/I code.
/// This function splits the input string into meaningful tokens such as strings, directives, and operators.
/// 
/// # Arguments
/// - `text`: A reference to a string slice containing the line to tokenize.
///
/// # Returns
/// A `Vec<String>` containing the tokens extracted from the input line.
fn tokenize_pli(text: &str) -> Vec<String> {
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
            tokens.push(current_token.clone()); // Save the directive token.
            current_token.clear();
        } else if c.is_whitespace() {
            // End of a token when whitespace is encountered.
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else if "!@#$%^&*()-+=[]{}|\\:;,.<>?/".contains(c) {
            // Handle special characters as individual tokens.
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            tokens.push(c.to_string()); // Save the special character as a token.
        } else {
            // Build regular tokens.
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token); // Save the last token if any.
    }

    tokens
}

/// Validates the file extension to ensure only `.pp` and `.pli` files are accepted.
///
/// # Arguments
/// - `input_file`: The name of the input file as a string slice.
///
/// # Returns
/// `true` if the file extension is valid, otherwise `false`.
fn validate_file_extension(input_file: &str) -> bool {
    let allowed_extensions = ["pp", "pli"]; // List of valid extensions.
    if let Some(extension) = Path::new(input_file).extension() {
        // Check if the file's extension matches any of the allowed ones.
        return allowed_extensions.contains(&extension.to_str().unwrap_or(""));
    }
    false
}

/// Determines whether a line starts with a valid preprocessor directive.
///
/// # Arguments
/// - `tokens`: A reference to a vector of tokens.
///
/// # Returns
/// `true` if the first token matches a valid directive, otherwise `false`.
fn is_valid_preprocessor_directive(tokens: &[String]) -> bool {
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
fn has_tokenizer_error(tokens: &[String]) -> bool {
    tokens.iter().any(|token| token.starts_with("'") && !token.ends_with("'"))
}

/// Processes the input file line by line, tokenizing and logging results.
///
/// # Arguments
/// - `input_file`: Path to the input file.
/// - `log_file`: Path to the log file where results are written.
///
/// # Returns
/// A `Result` indicating success or an I/O error.
fn process_file(input_file: &str, log_file: &str) -> io::Result<()> {
    let path = Path::new(input_file); // Create a `Path` object for the input file.
    let log_path = Path::new(log_file); // Create a `Path` object for the log file.

    let file = File::open(&path)?; // Open the input file (errors propagate with `?`).
    let reader = io::BufReader::new(file); // Use a buffered reader for efficiency.
    let mut log = File::create(&log_path)?; // Create or overwrite the log file.

    let mut total_lines = 0; // Counter for total lines processed.
    let mut pass_count = 0; // Counter for successful lines.
    let mut fail_count = 0; // Counter for failed lines.

    writeln!(log, "Processing file: {}", input_file)?; // Log the file being processed.

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue; // Skip blank lines.
        }

        total_lines += 1;

        let tokens = tokenize_pli(&line); // Tokenize the line.
        writeln!(log, "Line {}: {:?}", line_number + 1, tokens)?; // Log the tokens.

        if has_tokenizer_error(&tokens) {
            writeln!(log, "Line {}: FAIL (Tokenizer Error)", line_number + 1)?;
            fail_count += 1;
        } else if is_valid_preprocessor_directive(&tokens) {
            writeln!(log, "Line {}: PASS", line_number + 1)?;
            pass_count += 1;
        } else if !tokens.is_empty() {
            writeln!(log, "Line {}: Non-preprocessor line", line_number + 1)?;
        } else {
            writeln!(log, "Line {}: FAIL", line_number + 1)?;
            fail_count += 1;
        }
    }

    // Log summary statistics.
    writeln!(log, "\nSummary:")?;
    writeln!(log, "Total lines processed: {}", total_lines)?;
    writeln!(log, "Passes: {}", pass_count)?;
    writeln!(log, "Failures: {}", fail_count)?;

    Ok(())
}

/// The entry point of the program.
/// - Reads the input file path from command-line arguments.
/// - Validates the file extension.
/// - Processes the file and writes results to a log file.
fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments into a vector.

    if args.len() != 2 {
        eprintln!("Usage: pli_tokenizer <input_file>");
        std::process::exit(1); // Exit if the number of arguments is incorrect.
    }

    let input_file = &args[1];
    let log_file = "tests/output/pli_tokenizer.log"; // Log file path.

    if !validate_file_extension(input_file) {
        eprintln!("Error: Unsupported file extension. Only .pp and .pli files are allowed.");
        std::process::exit(1); // Exit if the file extension is invalid.
    }

    match process_file(input_file, log_file) {
        Ok(_) => println!("Processing complete. Results written to {}", log_file),
        Err(e) => eprintln!("Error processing file: {}", e),
    }
}

