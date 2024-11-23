#![allow(unused_imports)]
////////////////////////////////////////////////////////////////////////////////
// PL/I Preprocessor Main Program
// -----------------------------------------------------------------------------
// Author: Jean-Pierre Sainfeld
// Assistant: ChatGPT
// Company: FirstLink Consulting Services (FLCS)
// Date: 11/17/2024
// -----------------------------------------------------------------------------
// Description:
// The main entry point for the PL/I Preprocessor project. This program reads
// PL/I source files containing preprocessor directives and normal PL/I
// statements, processes them using various modules, and writes the results to
// an output file and log file.
//
// Features:
// - Tokenizes lines from an input file.
// - Validates preprocessor directives.
// - Supports macro expansion, include resolution, conditional execution, and more.
// - Generates transformed output and detailed logs.
//
// Purpose:
// The main program orchestrates the modular PL/I Preprocessor project.
// It serves as a learning exercise to explore Rust while implementing a
// practical tool.
//
// Usage:
// $ cargo run <input_file> <output_file> <log_file> [--verbose] [--dry-run]
//
// The results will be written to the specified output and log files.
//
// Company Mission:
// At FirstLink Consulting Services (FLCS), we specialize in delivering
// innovative solutions for complex software challenges.
//
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

use pli_tokenizer::modules::{
    tokenizer::{tokenize_pli, is_valid_preprocessor_directive, has_tokenizer_error},
    validator,
    evaluator,
    macro_expander,
    include_handler,
    conditional,
    logger,
    output,
};

use std::env; // Handles command-line arguments.
use std::fs::File; // Enables file operations.
use std::io::{self, BufRead, Write}; // Provides buffered I/O utilities.
use std::path::Path; // Allows manipulation of file paths.
use chrono::Local; // For timestamps in logging.

/// Processes the input file line by line and applies the preprocessor workflow.
/// This includes tokenization, validation, macro expansion, conditional evaluation, and more.
///
/// # Arguments
/// - `input_file`: The path to the input PL/I file.
/// - `output_file`: The path to the file where processed output will be written.
/// - `log_file`: The path to the log file for detailed logs.
/// - `verbose`: A boolean flag to control detailed console output.
/// - `dry_run`: A boolean flag to simulate processing without writing output.
///
/// # Returns
/// A `Result` indicating success or an I/O error.
fn process_file(
    input_file: &str,
    output_file: &str,
    log_file: &str,
    verbose: bool,
    dry_run: bool,
) -> io::Result<()> {
    // Create `Path` objects for input, output, and log files.
    let path = Path::new(input_file);
    let log_path = Path::new(log_file);
    let output_path = Path::new(output_file);

    // Open the input file and create buffered readers and writers.
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut log = File::create(&log_path)?;
    let mut output = if dry_run {
        None // Do not create the output file if dry-run is enabled.
    } else {
        Some(File::create(&output_path)?)
    };

    // Log the processing start with a timestamp.
    let start_time = Local::now();
    writeln!(log, "Processing started: {}", start_time)?;

    // Iterate through each line in the input file.
    for (line_number, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                if content.trim().is_empty() {
                    continue; // Skip blank lines.
                }

                if verbose {
                    println!("Processing line {}: {}", line_number + 1, content);
                }

                // Phase 1: Tokenization
                let tokens = tokenize_pli(&content);
                writeln!(log, "Line {} Tokens: {:?}", line_number + 1, tokens)?;

                // Phase 2: Validation
                // TODO: Validate the syntax of the tokenized line.
                // if validator::validate_syntax(&tokens) {
                //     writeln!(log, "Line {}: Syntax Valid", line_number + 1)?;
                // } else {
                //     writeln!(log, "Line {}: Syntax Error", line_number + 1)?;
                //     continue; // Skip further processing for invalid lines.
                // }

                // Phase 3: Macro Expansion
                // TODO: Expand macros in the line.
                // macro_expander::expand_macro("...");

                // Phase 4: Expression Evaluation
                // TODO: Evaluate conditional expressions in the line.
                // evaluator::evaluate_expression("...");

                // Phase 5: Include Resolution
                // TODO: Resolve includes to replace lines dynamically.
                // include_handler::handle_include("...");

                // Phase 6: Conditional Execution
                // TODO: Process conditional statements.
                // conditional::process_condition("...");

                // Phase 7: Output Generation
                if let Some(ref mut output_file) = output {
                    writeln!(output_file, "{}", content)?; // Write processed line to output file.
                }
            }
            Err(e) => {
                writeln!(log, "Error reading line {}: {}", line_number + 1, e)?;
            }
        }
    }

    // Log processing completion with a timestamp.
    let end_time = Local::now();
    writeln!(log, "Processing completed: {}", end_time)?;
    writeln!(log, "Output written to: {}", output_file)?;

    if verbose {
        println!("Processing completed. Log written to: {}", log_file);
    }

    Ok(())
}

/// Entry point for the PL/I Preprocessor program.
/// Handles command-line arguments and coordinates the workflow.
fn main() {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Ensure the correct number of arguments are provided.
    if args.len() < 4 || args.len() > 6 {
        eprintln!(
            "Usage: pli_preprocessor <input_file> <output_file> <log_file> [--verbose] [--dry-run]"
        );
        std::process::exit(1);
    }

    // Extract input, output, and log file paths from arguments.
    let input_file = &args[1];
    let output_file = &args[2];
    let log_file = &args[3];

    // Check for optional flags.
    let verbose = args.contains(&"--verbose".to_string());
    let dry_run = args.contains(&"--dry-run".to_string());

    // Validate the input file's extension.
    let allowed_extensions = ["pp", "pli"];
    if !allowed_extensions.iter().any(|ext| input_file.ends_with(ext)) {
        eprintln!("Error: Unsupported input file extension. Only .pp and .pli files are allowed.");
        std::process::exit(1);
    }

    // Process the file and handle any errors.
    match process_file(input_file, output_file, log_file, verbose, dry_run) {
        Ok(_) => println!("Processing complete."),
        Err(e) => eprintln!("Error processing file: {}", e),
    }
}

