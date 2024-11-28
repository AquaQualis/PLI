# Prompt Log for PL/I Tokenizer Development

This file contains guidelines, best practices, and prompts to ensure consistency and efficiency in the development and testing of the PL/I tokenizer and related modules.

---

## **General Guidelines**
1. **Documentation**:
   - Every function must have a detailed header comment.
   - Inline comments should explain non-trivial logic to ease comprehension.

2. **Code Clarity**:
   - Prefer `match` over multiple `if` statements for cleaner and more readable decision logic.
   - Ensure variable names are descriptive and meaningful.

3. **Consistency**:
   - Use `cargo fmt` to format code before committing changes.
   - Follow consistent naming conventions across modules (e.g., `snake_case` for functions, `CamelCase` for enums).

4. **Workflow**:
   - Use `git` for incremental checkpoints:
     - Check in documentation and resolved code as baselines.
     - Commit after each significant, self-contained change.
   - Use this prompt log to ensure adherence to best practices.

---

## **Module-Specific Guidelines**

### **Tokenizer Module**
1. Tokenization Process:
   - Ensure `tokenize_pli` properly splits input into meaningful tokens.
   - Handle edge cases like unmatched strings or malformed input.

2. Functions:
   - `handle_string_literal`: Must ensure string literals are captured entirely and handle unmatched quotes gracefully.
   - `handle_special_characters`: Categorize special characters (`;`, `#`, `*`, etc.) accurately.
   - `handle_directive`: Validate directives and associate them with their respective categories.

---

### **Testing Guidelines**
1. **Testing Workflow**:
   - Focus on a single test (e.g., `test_complex_inputs`) until it passes.
   - Debug failing tests incrementally, with debug logs where needed.

2. **Test Coverage**:
   - Include tests for:
     - Valid and invalid inputs.
     - Edge cases (e.g., empty lines, long lines, nested directives).
   - Ensure meaningful assertion messages for easy debugging.

3. **Execution**:
   - Use `cargo test --test <test_name> -- --nocapture` to view detailed test logs.

---

## **Workflow Prompts**
- Before making changes:
  - "Are all current changes checked into `git`?"
  - "Has `cargo fmt` been run?"
- During testing:
  - "What is the specific failure? Debug incrementally."
  - "Refer to this file for resolved decisions, such as preferring `match` constructs."

---

## **Resolved Issues**
1. **Using `match` Constructs**:
   - Replace redundant `if` statements with `match` for readability.
   - Example:
     ```rust
     let token_category = match c {
         '=' | ';' | '#' | '*' => TokenCategory::Operator,
         _ => TokenCategory::Unknown,
     };
     ```
2. **Tokenization Debugging**:
   - Use debug logs in `tokenize_pli` to trace the tokenization process.

3. **Handling Directives**:
   - Properly associate directives like `%IF` and `%THEN` with their categories.
   - Validate directives using `directive_categories`.

---

## **References**
- [Rust Documentation](https://doc.rust-lang.org/)
- [Cargo Command Cheatsheet](https://doc.rust-lang.org/cargo/commands/index.html)


## Documentation Template for Function Headers
"Use this template for all function documentation to ensure consistency and clarity in the codebase."


Every function in the PL/I tokenizer project should adhere to the following structured format for its comments. This ensures consistency, clarity, and ease of understanding.

rust
Copy code
////////////////////////////////////////////////////////////////////////////////
// FUNCTION: <Function Name>
// -----------------------------------------------------------------------------
// <Brief Description>
// 
// <Detailed Description>
// - Include the purpose of the function and when/why it is used.
// - Highlight any unique behavior or considerations (e.g., edge cases handled).
// - Provide an overview of the logic or flow, if complex.
//
// -----------------------------------------------------------------------------
// PARAMETERS:
// -----------------------------------------------------------------------------
// - `<Parameter Name>` (`<Type>`): <Description>
//   - <Additional details, if needed>
//
// -----------------------------------------------------------------------------
// RETURNS:
// -----------------------------------------------------------------------------
// - `<Type>`: <Description>
//   - <Additional details, if needed>
//
// -----------------------------------------------------------------------------
// ERRORS/CONDITIONS:
// -----------------------------------------------------------------------------
// - <Error/Condition>: <Description>
//   - <Additional details, if needed>
//
// -----------------------------------------------------------------------------
// SEE ALSO:
// -----------------------------------------------------------------------------
// - `<Related Function or Section>`: <Description or notes about the relation>
//
// -----------------------------------------------------------------------------
// EXAMPLE:
// -----------------------------------------------------------------------------
// ```rust
// <Code Example Here>
// ```
////////////////////////////////////////////////////////////////////////////////
Template Example
Here’s how the template could look when applied to a real function:

rust
Copy code
////////////////////////////////////////////////////////////////////////////////
// FUNCTION: tokenize_pli
// -----------------------------------------------------------------------------
// Tokenizes a given PL/I input string into a vector of categorized tokens.
//
// This function processes a single line of PL/I code and breaks it into tokens
// based on characters like whitespace, string literals, and special characters. 
// Debug logs are included to track the tokenization process.
// 
// Handles the following cases:
// - Whitespace between tokens.
// - String literals enclosed in single quotes.
// - Special characters like `;`, `#`, `*`.
//
// -----------------------------------------------------------------------------
// PARAMETERS:
// -----------------------------------------------------------------------------
// - `input` (`&str`): The PL/I input line to be tokenized.
//
// -----------------------------------------------------------------------------
// RETURNS:
// -----------------------------------------------------------------------------
// - `Vec<Token>`: A vector of tokens representing the parsed elements of the
//   input line.
//
// -----------------------------------------------------------------------------
// ERRORS/CONDITIONS:
// -----------------------------------------------------------------------------
// - Malformed Input: Unmatched string literals or invalid characters may cause 
//   unexpected behavior.
//
// -----------------------------------------------------------------------------
// SEE ALSO:
// -----------------------------------------------------------------------------
// - `handle_string_literal`: Handles tokenization of string literals.
// - `handle_special_characters`: Processes special characters.
//
// -----------------------------------------------------------------------------
// EXAMPLE:
// -----------------------------------------------------------------------------
// ```rust
// let input = "%IF DEBUG = 1 %THEN;";
// let tokens = tokenize_pli(input);
// assert_eq!(tokens[0].value, "%IF");
// assert_eq!(tokens[1].value, "DEBUG");
// ```
////////////////////////////////////////////////////////////////////////////////
