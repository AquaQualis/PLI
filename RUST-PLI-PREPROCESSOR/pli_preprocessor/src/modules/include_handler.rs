#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Include Handler
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This module handles the inclusion of external files in the PL/I preprocessor.
// It resolves `%INCLUDE` directives by locating, reading, and returning the content
// of specified files.
//
// FUNCTIONALITY:
// - Processes `%INCLUDE` directives in PL/I source code.
// - Validates the existence and readability of included files.
// - Supports relative and absolute paths.
//
// USAGE:
// - Use `process_include` to handle `%INCLUDE` directives.
// - Extend `resolve_include_path` to customize file path resolution.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
// VERSION: 1.0.0
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// IMPORTS
////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::path::{Path, PathBuf};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

/// Processes an `%INCLUDE` directive and returns the content of the included file.
///
/// # Arguments
/// - `directive`: A `&str` containing the `%INCLUDE` directive (e.g., `%INCLUDE 'file.pli';`).
/// - `current_dir`: A `&Path` representing the current working directory for relative paths.
///
/// # Returns
/// - `Result<String, String>`: Returns the file content as a string, or an error message.
///
/// # Example
/// ```rust
/// let content = process_include("%INCLUDE 'example.pli';", Path::new("/path/to/current"));
/// assert!(content.is_ok());
/// ```
pub fn process_include(directive: &str, current_dir: &Path) -> Result<String, String> {
    let file_path = extract_file_path(directive)
        .ok_or_else(|| format!("Invalid include directive: {}", directive))?;

    let resolved_path = resolve_include_path(&file_path, current_dir)?;

    read_file(&resolved_path)
}

/// Extracts the file path from an `%INCLUDE` directive.
///
/// # Arguments
/// - `directive`: A `&str` containing the `%INCLUDE` directive.
///
/// # Returns
/// - `Option<String>`: Returns the file path as a string, or `None` if the directive is invalid.
///
/// # Example
/// ```rust
/// let path = extract_file_path("%INCLUDE 'example.pli';");
/// assert_eq!(path, Some("example.pli".to_string()));
/// ```
pub fn extract_file_path(directive: &str) -> Option<String> {
    let parts: Vec<&str> = directive.split_whitespace().collect();

    // Ensure the directive starts with "%INCLUDE" and has at least two parts
    if parts.len() < 2 || parts[0] != "%INCLUDE" {
        return None;
    }

    // Trim leading/trailing quotes and semicolon
    let path = parts[1].trim_matches(&['\'', ';'][..]);

    // Return None if the path is empty after trimming
    if path.is_empty() {
        return None;
    }

    Some(path.to_string())
}

/// Resolves the full path of an included file.
pub fn resolve_include_path(file_path: &str, current_dir: &Path) -> Result<PathBuf, String> {
    let path = Path::new(file_path);
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(current_dir.join(path))
    }
}

/// Reads the content of a file.
pub fn read_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|err| format!("Failed to read file {}: {}", path.display(), err))
}
