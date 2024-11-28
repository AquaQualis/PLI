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

