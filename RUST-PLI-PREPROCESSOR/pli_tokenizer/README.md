# PL/I Preprocessor in Rust

This project implements a PL/I preprocessor in Rust. It includes tokenization, file validation, and basic syntax handling for PL/I preprocessor directives such as `%IF`, `%THEN`, `%ELSE`, `%ENDIF`, and `%COMMENT`.

## Features Implemented
1. **Tokenization**:
   - Handles PL/I syntax such as strings, preprocessor directives, and operators.
   - Supports quoted strings, comments, and special characters.

2. **File Validation**:
   - Accepts only files with `.pp` and `.pli` extensions.
   - Logs unsupported file extensions with a warning.

3. **Pass/Fail Logic**:
   - Differentiates valid preprocessor lines, non-preprocessor lines, and blank lines.
   - Skips blank lines and logs tokenizer errors.

4. **Support for Basic Directives**:
   - `%IF`, `%THEN`, `%ELSE`, `%ENDIF`, `%COMMENT`.

5. **Testing Framework**:
   - Includes a `tests/input/` directory with sample test files.

## Project Structure
pli_preprocessor/ ├── src/ │ └── main.rs # Rust source code ├── tests/ │ ├── input/ # Input test files │ │ ├── valid_file.pp │ │ ├── invalid_file.txt │ │ ├── edge_case.pli │ │ └── if_example.pp │ └── output/ # Optional directory for generated log files ├── pli_tokenizer.log # Example log file (optional) ├── Cargo.toml # Rust project configuration ├── README.md # Project documentation └── .gitignore # Git ignore rules


## How to Run
1. Clone the repository:
   ```bash
   git clone <your-repo-url>
   cd pli_preprocessor

    Build and run the program:

cargo build
cargo run tests/input/valid_file.pp

Check the log file:

    cat pli_tokenizer.log

Test Files

    valid_file.pp: Contains valid PL/I preprocessor directives.
    invalid_file.txt: Invalid file extension for rejection testing.
    edge_case.pli: Tests nested and complex directives.
    if_example.pp: Demonstrates %IF, %THEN, %ELSE, and nested logic.

Next Steps

    Expand tokenizer to support additional directives like %DO and %INCLUDE.
    Add robust error handling for invalid syntax.
    Implement preprocessor expressions for evaluating %IF conditions.

Contributions

Feel free to open an issue or submit a pull request!


---

### **Capturing Work with Git**
1. **Initialize Git (if not done already):**
   ```bash
   git init

    Add Files to the Repo:

git add src/main.rs tests/input/ README.md Cargo.toml .gitignore

Commit the Changes:

git commit -m "Initial commit: Tokenizer and basic preprocessor functionality"

Push to GitHub:

    git branch -M main
    git remote add origin <your-repo-url>
    git push -u origin main

Capturing Progress

Include notes or commit messages that document the steps we've completed:

    Tokenization: Handles strings, directives, and special characters.
    Validation: Processes only .pp and .pli files.
    Pass/Fail Logic: Differentiates preprocessor directives from normal lines.
    Test Inputs: Added input files for testing.
