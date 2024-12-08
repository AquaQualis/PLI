//! @file utils.rs
//! @brief Utility functions for the PL/I Preprocessor.
//!
//! This module provides utility functions that support various operations
//! across the PL/I Preprocessor. These functions are designed to be modular
//! and reusable, aiding in common tasks such as string manipulation,
//! error handling, and logging enhancements.

use log::{Level, LevelFilter};
use fern::Dispatch;
use std::sync::Once;

static INIT: Once = Once::new();

/// Initializes the logger for the preprocessor.
pub fn initialize_logger() {
    INIT.call_once(|| {
        let result = fern::Dispatch::new()
            .level(LevelFilter::Debug) // Set maximum level to DEBUG
            .level_for("pli_preprocessor", LevelFilter::Debug) // Scope-specific configuration
            .chain(std::io::stdout())
            .apply();

        if let Err(e) = result {
            eprintln!("Logger initialization failed: {:?}", e);
        }
        println!("Logger initialized with max level: {:?}", LevelFilter::Debug);
    });
}

/// Converts a string to uppercase.
pub fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

/// Determines if a log level is enabled.
pub fn is_log_level_enabled(level: Level) -> bool {
    log::log_enabled!(level)
}

/// Joins a vector of strings into a single string with a specified delimiter.
pub fn join_with_delimiter(items: Vec<&str>, delimiter: &str) -> String {
    items.join(delimiter)
}

/// Checks if a string is blank (empty or contains only whitespace).
pub fn is_blank(input: &str) -> bool {
    input.trim().is_empty()
}

/// Splits a string into words, preserving quoted substrings.
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
    use log::Level;

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
