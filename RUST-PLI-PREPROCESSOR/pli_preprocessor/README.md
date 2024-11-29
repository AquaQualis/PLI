PL/I Preprocessor in Rust
A robust and extensible PL/I preprocessor implemented in Rust, featuring tokenization, syntax validation, and preprocessor directive handling. Designed for performance, scalability, and ease of use, this project supports core PL/I preprocessing logic while offering extensive test coverage.

📚 Features
1. Tokenization
Accurately handles PL/I syntax including:
Strings ('example')
Preprocessor directives (%IF, %THEN, %ELSE, %ENDIF)
Special characters (!@#$%^&*()-+=)
Combines special characters with identifiers (@INVALID_CHAR).
2. File Validation
Processes only files with .pp or .pli extensions.
Logs unsupported file extensions with detailed warnings.
3. Directive Handling
Supports the following preprocessor directives:
%IF, %THEN, %ELSE, %ENDIF, %COMMENT
Differentiates between valid, invalid, and non-preprocessor lines.
Skips blank lines and logs errors during tokenization.
4. Test Suite
Comprehensive testing framework with 13 passing test cases.
Tests include:
Valid files.
Invalid files.
Edge cases (e.g., nested and deeply nested directives).
Mixed content and very large files.
5. Logging
Outputs tokenization and validation results to a log file (pli_tokenizer.log).
📂 Project Structure
plaintext
Copy code
pli_preprocessor/
├── src/
│   └── main.rs               # Main Rust source code.
├── tests/
│   ├── input/                # Test input files.
│   │   ├── valid_file.pp     # Valid PL/I directives.
│   │   ├── invalid_file.txt  # Invalid file extension test case.
│   │   ├── edge_case.pli     # Complex directive cases.
│   │   └── if_example.pp     # Nested logic test.
│   └── output/               # Optional output directory for logs.
├── pli_tokenizer.log         # Example log file.
├── Cargo.toml                # Rust project configuration.
├── README.md                 # Project documentation.
└── .gitignore                # Git ignore rules.
⚙️ How to Use
1. Clone the Repository
bash
Copy code
git clone <your-repo-url>
cd pli_preprocessor
2. Build the Project
bash
Copy code
cargo build
3. Run the Preprocessor
bash
Copy code
cargo run tests/input/valid_file.pp
4. View Logs
bash
Copy code
cat pli_tokenizer.log
5. Run Tests
bash
Copy code
cargo test -- --nocapture
🧪 Test Files
valid_file.pp: Contains valid PL/I preprocessor directives.
invalid_file.txt: Tests rejection of unsupported file extensions.
edge_case.pli: Tests nested and complex directives.
if_example.pp: Demonstrates %IF, %THEN, %ELSE, and nested logic.
🚀 Next Steps
Planned Enhancements
Expand Directive Support:
Add %DO and %INCLUDE.
Error Handling:
Implement robust error detection for invalid syntax.
Expressions:
Add support for preprocessor expressions in %IF conditions.
Code Refactoring:
Replace if-else logic in tokenize_pli with a match statement for readability and maintainability.
Contributions
Contributions are welcome! Open an issue or submit a pull request with your suggestions or fixes.
📋 Capturing Work with Git
1. Initialize Git
bash
Copy code
git init
2. Add Files
bash
Copy code
git add src/main.rs tests/input/ README.md Cargo.toml .gitignore
3. Commit Changes
bash
Copy code
git commit -m "Initial commit: Tokenizer and basic preprocessor functionality"
4. Push to GitHub
bash
Copy code
git branch -M main
git remote add origin <your-repo-url>
git push -u origin main
📝 Progress Tracking
Features Completed
Tokenization: Handles strings, directives, and special characters.
Validation: Processes .pp and .pli files only.
Pass/Fail Logic: Differentiates directives from plain text lines.
Test Inputs: Comprehensive test coverage with 13 test cases.
Pending Work
Implement advanced directive support.
Refactor for improved readability using match.
💡 About This Project
At FirstLink Consulting Services (FLCS), we specialize in delivering innovative software solutions. This project showcases our commitment to building robust, maintainable, and high-performance tools.
