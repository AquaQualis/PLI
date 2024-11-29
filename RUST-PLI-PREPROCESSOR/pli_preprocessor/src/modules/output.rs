#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Output Handler
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This module handles the output operations for the PL/I preprocessor.
// It supports writing processed code, logs, and error messages to files or other
// destinations.
//
// FUNCTIONALITY:
// - Writes processed lines of code to an output file.
// - Appends logs or debug messages to a designated log file.
// - Ensures proper handling of file creation, opening, and closing.
// - Handles errors gracefully during file operations.
//
// USAGE:
// - Use `write_line_to_file` to write a single line to an output file.
// - Use `append_log_message` to add a log entry to a log file.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
// VERSION: 1.0.0
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// IMPORTS
////////////////////////////////////////////////////////////////////////////////

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

/// Writes a single line to an output file, creating or overwriting the file.
///
/// # Arguments
/// - `file_path`: The path to the output file.
/// - `line`: The line of text to write.
///
/// # Returns
/// - `Result<(), String>`: Returns `Ok(())` if successful, or an error message.
///
/// # Example
/// ```rust
/// write_line_to_file("/tmp/output.txt", "Processed line").unwrap();
/// ```
pub fn write_line_to_file(file_path: &Path, line: &str) -> Result<(), String> {
    let mut file = File::create(file_path)
        .map_err(|e| format!("Failed to create file {}: {}", file_path.display(), e))?;
    file.write_all(line.as_bytes())
        .map_err(|e| format!("Failed to write to file {}: {}", file_path.display(), e))
}

/// Appends a log message to a log file, creating the file if it does not exist.
///
/// # Arguments
/// - `log_path`: The path to the log file.
/// - `message`: The log message to append.
///
/// # Returns
/// - `Result<(), String>`: Returns `Ok(())` if successful, or an error message.
///
/// # Example
/// ```rust
/// append_log_message("/tmp/preprocessor.log", "Log entry").unwrap();
/// ```
pub fn append_log_message(log_path: &Path, message: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .map_err(|e| format!("Failed to open log file {}: {}", log_path.display(), e))?;
    writeln!(file, "{}", message).map_err(|e| {
        format!(
            "Failed to write log message to {}: {}",
            log_path.display(),
            e
        )
    })
}
