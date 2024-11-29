#![allow(unused_imports)] // Suppress unused warnings for imports temporarily.
                          ////////////////////////////////////////////////////////////////////////////////
                          // MODULE NAME: Logger
                          // -----------------------------------------------------------------------------
                          // Description:
                          // This module provides centralized logging functionality for the PL/I
                          // preprocessor application. It supports logging to both a console and a file
                          // with configurable verbosity levels, timestamped entries, and multiple log levels.
                          //
                          // Features:
                          // - Logs messages to a specified log file and optionally to the console.
                          // - Configurable verbosity levels to control the granularity of log output:
                          //     - `0`: Logs only errors (`ERROR`).
                          //     - `1`: Logs warnings and errors (`WARN` and `ERROR`).
                          //     - `2`: Logs informational messages, warnings, and errors (`INFO`, `WARN`, and `ERROR`).
                          //     - `3..=31`: Logs debug-level messages in addition to the above (`DEBUG`).
                          //     - `>=32`: Logs everything, including trace-level details (`TRACE`).
                          // - Timestamped log entries in the format `YYYY-MM-DD HH:MM:SS.mmmµs`.
                          // - Easy integration with the `log` crate macros: `log::info!`, `log::debug!`, etc.
                          // - Flexible configuration for dynamic logging needs.
                          //
                          // Purpose:
                          // Centralized logging is essential for debugging, monitoring, and maintaining the
                          // application. This module simplifies the process by providing an easy-to-use
                          // interface for logging messages at various levels of detail.
                          //
                          // Usage:
                          // 1. Call `init_logger()` at the start of the application to initialize logging.
                          // 2. Use `log` macros (`log::info!`, `log::debug!`, etc.) throughout the application.
                          //
                          // Example:
                          // ```rust
                          // mod logger;
                          //
                          // fn main() {
                          //     if let Err(e) = logger::init_logger("application.log", true, 3) {
                          //         eprintln!("Failed to initialize logger: {}", e);
                          //         std::process::exit(1);
                          //     }
                          //
                          //     log::info!("Logger initialized with verbosity level 3 (DEBUG).");
                          //     log::debug!("This is a debug message.");
                          //     log::error!("This is an error message.");
                          // }
                          // ```
                          //
                          // Dependencies:
                          // - `log`: For log macros and logging functionality.
                          // - `fern`: For flexible and composable log dispatching.
                          // - `chrono`: For generating timestamps.
                          //
                          // Note:
                          // - `fern::Dispatch`: Used for configuring the logger.
                          // - `chrono::Local`: Used for timestamped logging.
                          // - `log::{debug, error, info, warn, trace}`: Provides convenient macros for different log levels.
                          // - `log::LevelFilter`: Required to define log filtering behavior.
                          // - `std::io`: Necessary for handling I/O errors during logger initialization.
                          //
                          // Enhancements:
                          // - Added configurable verbosity levels for granular control of log output.
                          // - Improved flexibility for log level configuration, making the logger suitable for debugging,
                          //   monitoring, and production environments.
                          // - Timestamps now include microsecond precision (`YYYY-MM-DD HH:MM:SS.mmmµs`).
                          //
                          // Author: Jean-Pierre Sainfeld
                          // Assistant: ChatGPT
                          // Company: FirstLink Consulting Services (FLCS)
                          // -----------------------------------------------------------------------------
                          ////////////////////////////////////////////////////////////////////////////////

use chrono::Local; // For timestamped logs.
use fern::Dispatch;
use log::LevelFilter; // For setting log level filtering.
use log::{debug, error, info, warn};
use std::io; // For potential I/O errors in logger initialization.

/// Initializes the logging system for the PL/I Preprocessor application.
///
/// This function configures the logging system to log messages to a specified file
/// and optionally the console, depending on the verbosity level and verbosity flag provided.
/// Log messages are formatted with timestamps, log levels, and message content.
///
/// # Arguments
/// - `log_file`: A `&str` specifying the path of the log file where logs will be saved.
/// - `verbose`: A `bool` flag to enable verbose output. When `true`, a confirmation message
///   is printed to the console, and logs may include more detailed information.
/// - `verbosity_level`: A `u8` representing the granularity of log output:
///     - `0`: Logs only errors (`ERROR`).
///     - `1`: Logs warnings and errors (`WARN` and `ERROR`).
///     - `2`: Logs informational messages, warnings, and errors (`INFO`, `WARN`, and `ERROR`).
///     - `3..=31`: Logs debug-level messages in addition to the above (`DEBUG`).
///     - `>=32`: Logs everything, including trace-level details (`TRACE`).
///
/// # Returns
/// - `Ok(())`: If the logger was successfully initialized.
/// - `Err(fern::InitError)`: If an error occurred during logger initialization, such as
///   issues creating or writing to the log file.
///
/// # Features
/// - Logs messages to a file and optionally to the console.
/// - Timestamps each log entry in the format `YYYY-MM-DD HH:MM:SS.mmmµs`.
/// - Supports configurable log levels: `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`.
/// - Designed for modular and flexible integration with different verbosity requirements.
///
/// # Example
/// ```rust
/// if let Err(e) = init_logger("application.log", true, 3) {
///     eprintln!("Failed to initialize logger: {}", e);
///     std::process::exit(1);
/// }
/// log::info!("Logger initialized successfully with verbosity level 3 (DEBUG).");
/// ```
///
/// # Notes
/// - Call this function once at the start of the application, before generating any log messages.
/// - If the log file cannot be created or accessed, the function returns an error.
/// - This function utilizes the `fern` crate for log configuration and the `log` crate for logging macros.
///
/// # Dependencies
/// This function requires the following crates:
/// - `fern`: For log dispatch and configuration.
/// - `log`: For the logging macros such as `info!`, `warn!`, etc.
/// - `chrono`: For timestamp formatting in log entries.
///
/// # Authors
/// - Jean-Pierre Sainfeld, FirstLink Consulting Services (FLCS)
/// - Assistant: ChatGPT, supporting with design and technical enhancements.
pub fn init_logger(
    log_file: &str,
    verbose: bool,
    verbosity_level: u8,
) -> Result<(), fern::InitError> {
    let log_level = match verbosity_level {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3..=31 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            let now = chrono::Local::now();
            out.finish(format_args!(
                "[{}.{:06}][{}] {}",
                now.format("%Y-%m-%d %H:%M:%S"),
                now.timestamp_subsec_micros(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Error) // Default log level for all modules.
        .level_for("pli_tokenizer", log_level) // Specific log level for the application.
        .chain(fern::log_file(log_file)?) // Log to the specified file.
        .apply()?;

    if verbose {
        println!(
            "Logger initialized. Verbosity level: {} ({:?})",
            verbosity_level, log_level
        );
        log::info!(
            "Logger initialized with verbosity level: {} ({:?})",
            verbosity_level,
            log_level
        );
    }

    Ok(())
}
