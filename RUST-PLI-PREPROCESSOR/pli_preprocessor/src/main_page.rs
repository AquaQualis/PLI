//! @mainpage PL/I Preprocessor Documentation
//!
//! @section intro_sec Introduction
//! The PL/I Preprocessor is a Rust-based tool for handling preprocessor directives
//! in PL/I source code. This project is designed to tokenize, validate, and transform
//! PL/I source files, providing a foundation for compiling or interpreting PL/I code
//! while learning and leveraging the power of Rust.
//!
//! @section features_sec Features
//! - Tokenizes PL/I source code, breaking it into structured tokens.
//! - Validates preprocessor directives for syntax correctness.
//! - Supports macro expansion, include directives, and conditional statements.
//! - Handles multiline directives and complex expressions with operator precedence.
//! - Generates detailed logs and transformed output files.
//!
//! @section usage_sec Usage
//! Run the preprocessor with the following command:
//! ```bash
//! cargo run <input_file> <output_file> <log_file> [--verbose] [--dry-run] [--verbosity=<level>]
//! ```
//!
//! ### Positional Arguments
//! - `<input_file>`: Path to the input PL/I source file.
//! - `<output_file>`: Path to the output file where transformed content will be written.
//! - `<log_file>`: Path to the log file for detailed logs.
//!
//! ### Optional Flags
//! - `--verbose`: Enables additional console output.
//! - `--dry-run`: Simulates processing without writing output.
//! - `--verbosity=<level>`: Configures log verbosity level.
//!
//! @section modules_sec Modules
//! - **Parser Module**: Handles tokenization, control structure parsing, and expression parsing.
//! - **Logger Module**: Manages logging operations with configurable verbosity levels.
//! - **Tokenizer Module**: Splits PL/I source code into structured tokens.
//! - **Macro Expander**: Expands macros and resolves include directives.
//!
//! @section contrib_sec Contributions
//! Author: Jean-Pierre Sainfeld  
//! Assistant: ChatGPT  
//! Company: FirstLink Consulting Services (FLCS)
//!
//! @section license_sec License
//! This project is for educational and non-commercial purposes only.
//!
//! ---
