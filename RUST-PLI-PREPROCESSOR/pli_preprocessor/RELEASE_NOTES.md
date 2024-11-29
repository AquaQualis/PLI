# PL/I Preprocessor - Release Notes

## Version v0.1.0-rc1 (2024-11-26)

### Overview
The **PL/I Preprocessor v0.1.0-rc1** release candidate delivers the foundational functionality required for preprocessing PL/I code. This initial milestone focuses on the tokenizer module, providing robust and extensible tokenization capabilities. 

This release establishes a solid base for future development, paving the way for advanced macro processing, conditional evaluation, and integration into the broader preprocessing pipeline.

---

### Key Features
#### **1. Tokenizer Module**
- **Core Functionality**:
  - Converts PL/I code into categorized tokens for further processing.
  - Categorizes tokens into:
    - **Directives**: `%IF`, `%THEN`, `%ELSE`, etc.
    - **Identifiers**: Variable names, functions, etc.
    - **Literals**: String constants and numeric values.
    - **Operators**: `=`, `#`, `*`, etc.
    - **Special Characters**: `;`, `(`, `)`, etc.
  - Handles edge cases like:
    - Unmatched quotes in string literals.
    - Malformed or unexpected input characters.

- **Error Detection**:
  - Identifies malformed tokens (e.g., unmatched string literals).
  - Validates directives to ensure they conform to PL/I syntax.

- **Extensibility**:
  - Modular design for easy integration with future preprocessing components.

#### **2. Comprehensive Testing Framework**
- **Unit Tests**:
  - Verify directive tokenization, including case-insensitive handling.
  - Validate token categorization for operators, string literals, and special characters.
  - Test edge cases like empty lines and malformed inputs.

- **Debugging Support**:
  - Includes detailed logs for tracking tokenization processes during tests.
  - Provides meaningful error messages for failed assertions.

---

### Improvements
- **Code Readability**:
  - Refactored logic using `match` constructs for concise and maintainable code.
  - Enhanced variable naming for clarity and context.

- **Documentation**:
  - Detailed function headers and inline comments for all core functions.
  - Maintained a **PROMPT_LOG.md** file documenting design decisions, guidelines, and workflow best practices.

- **Workflow Enhancements**:
  - Adopted Gitflow branching model for structured development.
  - Streamlined testing with `cargo test` and added debugging capabilities.

---

### Future Work
This release sets the stage for future functionality, including:
1. **Macro Processing**:
   - Implement `%MACRO` handling for reusable code blocks.

2. **Conditional Evaluation**:
   - Expand `%IF` and `%THEN` support to include nested and complex conditions.

3. **Performance Optimization**:
   - Fine-tune tokenizer performance for large-scale PL/I codebases.

4. **Expanded Testing**:
   - Develop integration tests for multi-line and complex PL/I statements.
   - Add tests for uncommon edge cases to improve reliability.

---

### Known Limitations
- **Feature Scope**:
  - Focuses solely on tokenization. Other preprocessing functionalities like macro expansion and condition evaluation are not included in this release.

- **Input Constraints**:
  - Limited support for multi-line directives and comments in this version.

---

### Acknowledgments
We extend our gratitude to the team members and contributors who made this release possible. Your efforts have laid the foundation for the comprehensive PL/I Preprocessor system.

---

### Contact and Support
For questions, feedback, or to report issues, please reach out to us:
- **Email**: [support@example.com](mailto:support@example.com)
- **Repository**: [GitHub Repo](https://github.com/your-repo-link)

---

### Appendix: Function Inventory
#### **Tokenizer Module**
- **`tokenize_pli`**: Splits input strings into tokens.
- **`handle_directive`**: Processes directives starting with `%`.
- **`handle_string_literal`**: Handles string literals enclosed in quotes.
- **`handle_special_characters`**: Tokenizes special characters like `;` and `=`.
- **`finalize_token`**: Finalizes the current token being constructed.
- **`has_tokenizer_error`**: Checks for tokenizer errors like unmatched strings.
- **`is_valid_preprocessor_directive`**: Validates the presence of a valid directive.

#### **Test Suite**
- **`test_case_insensitivity`**: Verifies case-insensitive handling of directives.
- **`test_handle_special_characters`**: Ensures correct tokenization of special characters.
- **`test_string_literals`**: Tests proper handling of string literals and unmatched quotes.
- **`test_edge_cases`**: Handles edge cases like empty input and unexpected characters.

---

## Instructions for Use
1. **Cloning the Repository**:
   ```bash
   git clone https://github.com/your-repo-link.git
   cd pli_preprocessor

