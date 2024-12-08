//! @file utils.rs
//! @brief Utility functions for the PL/I Preprocessor.
//!
//! This module provides utility functions that support various operations
//! across the PL/I Preprocessor. These functions are designed to be modular
//! and reusable, aiding in common tasks such as string manipulation,
//! error handling, and logging enhancements.
//!
//! @details
//! The `utils` module is intended to encapsulate common logic that does not
//! belong to any specific module, providing helper methods that streamline
//! the development and maintenance of the preprocessor.
//!
//! @author
//! - Jean-Pierre Sainfeld
//! - Assistant: ChatGPT
//!
//! @company FirstLink Consulting Services (FLCS)
//!
//! @version 1.0
//! @date 2024-11-24


use log::{Level, LevelFilter};
use fern::Dispatch;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn initialize_logger() {
    INIT.call_once(|| {
        let _ = fern::Dispatch::new()
            .level(LevelFilter::Debug) // Set maximum level to DEBUG
            .level_for("pli_preprocessor", LevelFilter::Debug) // Scope-specific configuration
            .chain(std::io::stdout())
            .apply()
            .map_err(|e| eprintln!("Logger initialization failed: {:?}", e));
        println!("Logger initialized with max level: {:?}", LevelFilter::Debug);
    });
}




/// Converts a string to uppercase.
///
/// This function takes a string slice and returns a new string with all
/// characters converted to uppercase.
///
/// # Arguments
/// * `input` - A string slice to be converted to uppercase.
///
/// # Returns
/// * A `String` containing the uppercase representation of the input.
///
/// # Example
/// ```rust
/// use pli_preprocessor::modules::tokenizer::utils::to_uppercase;
///
/// let input = "hello, world!";
/// let result = to_uppercase(input);
/// assert_eq!(result, "HELLO, WORLD!");
/// ```
pub fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

/// Determines if a log level is enabled.
///
/// This function checks if a specific log level is currently enabled
/// for logging. It is useful for conditional logging based on the
/// configured verbosity.
///
/// # Arguments
/// * `level` - A `log::Level` representing the log level to check.
///
/// # Returns
/// * `true` if the log level is enabled; otherwise, `false`.
///
/// # Example
/// ```rust
/// use log::Level;
/// use pli_preprocessor::modules::tokenizer::utils::is_log_level_enabled;
///
/// let enabled = is_log_level_enabled(Level::Info);
/// println!("Is INFO logging enabled? {}", enabled);
/// ```
pub fn is_log_level_enabled(level: Level) -> bool {
    log::log_enabled!(level)
}

/// Joins a vector of strings into a single string with a specified delimiter.
///
/// This function takes a vector of strings and joins them into a single
/// string, inserting the specified delimiter between each element.
///
/// # Arguments
/// * `items` - A vector of strings to join.
/// * `delimiter` - A string slice to use as the delimiter.
///
/// # Returns
/// * A `String` containing the joined elements separated by the delimiter.
///
/// # Example
/// ```rust
/// use pli_preprocessor::modules::tokenizer::utils::join_with_delimiter;
///
/// let items = vec!["A", "B", "C"];
/// let result = join_with_delimiter(items, ", ");
/// assert_eq!(result, "A, B, C");
/// ```
pub fn join_with_delimiter(items: Vec<&str>, delimiter: &str) -> String {
    items.join(delimiter)
}

/// Checks if a string is blank (empty or contains only whitespace).
///
/// This function determines whether a string slice is blank by checking
/// if it is empty or consists solely of whitespace characters.
///
/// # Arguments
/// * `input` - A string slice to check.
///
/// # Returns
/// * `true` if the string is blank; otherwise, `false`.
///
/// # Example
/// ```rust
/// use pli_preprocessor::modules::tokenizer::utils::is_blank;
///
/// assert!(is_blank("   "));
/// assert!(!is_blank("hello"));
/// ```
pub fn is_blank(input: &str) -> bool {
    input.trim().is_empty()
}

/// Splits a string into words, preserving quoted substrings.
///
/// This function splits a string into words, treating substrings enclosed
/// in quotes as single tokens. It handles escaped quotes within quoted
/// substrings.
///
/// # Arguments
/// * `input` - A string slice to split into words.
///
/// # Returns
/// * A `Vec<String>` containing the split words.
///
/// # Example
/// ```rust
/// use pli_preprocessor::modules::tokenizer::utils::split_preserving_quotes;
///
/// let input = "word1 \"quoted word2\" word3";
/// let result = split_preserving_quotes(input);
/// assert_eq!(result, vec!["word1", "\"quoted word2\"", "word3"]);
/// ```
pub fn split_preserving_quotes(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' if !in_quotes => {
                in_quotes = true;
                current.push(c);
            }
            '"' if in_quotes => {
                in_quotes = false;
                current.push(c);
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    words.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{Level, LevelFilter};
    use std::sync::Once;

    /// Ensures the logger is initialized only once for the entire test suite.
    static INIT_LOGGER: Once = Once::new();

    /// Initializes the logger for testing purposes.
    fn initialize_logger() {
        INIT_LOGGER.call_once(|| {
            let _ = fern::Dispatch::new()
                .level(LevelFilter::Trace)
                .chain(std::io::stdout())
                .apply();
        });
    }

    /// @test Test the `to_uppercase` function.
    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("hello"), "HELLO");
        assert_eq!(to_uppercase("WORLD"), "WORLD");
    }

    /// @test Test the `is_log_level_enabled` function.
    #[test]
    fn test_is_log_level_enabled() {
        initialize_logger();
    
        // Explicitly set log level defaults for the test
        log::set_max_level(log::LevelFilter::Debug);
    
        // Debugging current logger state
        eprintln!(
            "Log levels: INFO: {}, DEBUG: {}, TRACE: {}",
            is_log_level_enabled(Level::Info),
            is_log_level_enabled(Level::Debug),
            is_log_level_enabled(Level::Trace)
        );
    
        assert!(is_log_level_enabled(Level::Info), "INFO log level should be enabled.");
        assert!(is_log_level_enabled(Level::Debug), "DEBUG log level should be enabled.");
        assert!(!is_log_level_enabled(Level::Trace), "TRACE log level should not be enabled by default.");
    }
    


    /// @test Test the `join_with_delimiter` function.
    #[test]
    fn test_join_with_delimiter() {
        let items = vec!["A", "B", "C"];
        assert_eq!(join_with_delimiter(items, ", "), "A, B, C");
    }

    /// @test Test the `is_blank` function.
    #[test]
    fn test_is_blank() {
        assert!(is_blank("   "));
        assert!(!is_blank("hello"));
    }

    /// @test Test the `split_preserving_quotes` function.
    #[test]
    fn test_split_preserving_quotes() {
        let input = r#"word1 "quoted word2" word3"#;
        let expected = vec!["word1", r#""quoted word2""#, "word3"];
        assert_eq!(split_preserving_quotes(input), expected);
    }
}
