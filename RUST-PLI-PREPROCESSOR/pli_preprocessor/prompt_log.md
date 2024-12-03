Prompt Log for PL/I Tokenizer Development
This file contains guidelines, best practices, and prompts to ensure consistency and efficiency in the development and testing of the PL/I tokenizer and related modules.

General Guidelines
Documentation:

Every function must have a detailed header comment.
Inline comments should explain non-trivial logic to ease comprehension.
Code Clarity:

Prefer match over multiple if statements for cleaner and more readable decision logic.
Ensure variable names are descriptive and meaningful.
Consistency:

Use cargo fmt to format code before committing changes.
Follow consistent naming conventions across modules (e.g., snake_case for functions, CamelCase for enums).
Workflow:

Use git for incremental checkpoints:
Check in documentation and resolved code as baselines.
Commit after each significant, self-contained change.
Use this prompt log to ensure adherence to best practices.
Module-Specific Guidelines
Tokenizer Module
Tokenization Process:

Ensure tokenize_pli properly splits input into meaningful tokens.
Handle edge cases like unmatched strings or malformed input.
Core Functions:

handle_string_literal: Must ensure string literals are captured entirely and handle unmatched quotes gracefully.
handle_special_characters: Categorize special characters (;, #, *, etc.) accurately.
handle_directive: Validate directives and associate them with their respective categories.
finalize_token: Ensure proper finalization and addition of tokens.
Testing Guidelines
Testing Workflow:

Focus on a single test (e.g., test_complex_inputs) until it passes.
Debug failing tests incrementally, with debug logs where needed.
Test Coverage:

Include tests for:
Valid and invalid inputs.
Edge cases (e.g., empty lines, long lines, nested directives).
Seemingly minor but critical functionality (e.g., handle_directive).
Ensure meaningful assertion messages for easy debugging.
Execution:

Use cargo test --test <test_name> -- --nocapture to view detailed test logs.
Dependency Checklist
"When modifying a module, verify that all functions it depends on are intact and meet project guidelines. Functions should not be removed or renamed without validating their impact. Use tools like grep or IDE search to locate dependencies."

Change Impact Analysis
"If a function is removed or renamed:

List all references to it in the codebase using tools like grep or an IDE.
Validate that existing test coverage accounts for the functionality being removed.
Document the rationale for the change in the commit message."
Comprehensive Test Coverage
"Every function should have at least one dedicated test. Tests should:

Validate the function's core purpose.
Cover edge cases and error scenarios.
Be updated or added when functions are modified."
Comprehensive Function Inventory
Every module should include a function inventory at the top, listing all functions and their roles. This inventory should be updated with every significant change.

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: <Function/Test Name>
// -----------------------------------------------------------------------------
// Description:
// <Brief description of the purpose and functionality of the function/test>.
//
// Example:
// - What scenarios does this test cover, or what functionality does this
//   function implement?
//
// Inputs:
// - `parameter1` (`type`): <Description>
// - `parameter2` (`type`): <Description>
//
// Outputs:
// - `type`: <Description>
//
// -----------------------------------------------------------------------------


Example Function Inventory for tokenizer.rs:
rust
Copy code
////////////////////////////////////////////////////////////////////////////////
// FUNCTION INVENTORY
// -----------------------------------------------------------------------------
// - tokenize_pli: Splits input strings into tokens.
// - handle_directive: Processes directives starting with `%`.
// - handle_string_literal: Handles string literals enclosed in quotes.
// - handle_special_characters: Tokenizes special characters like `;` and `=`.
// - finalize_token: Finalizes the current token being constructed.
////////////////////////////////////////////////////////////////////////////////
Documentation Template for Function Headers
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
Resolved Issues
Using match Constructs:

Replace redundant if statements with match for readability.
Example:
rust
Copy code
let token_category = match c {
    '=' | ';' | '#' | '*' => TokenCategory::Operator,
    _ => TokenCategory::Unknown,
};
Tokenization Debugging:

Use debug logs in tokenize_pli to trace the tokenization process.
Handling Directives:

Properly associate directives like %IF and %THEN with their categories.
Validate directives using get_directive_category.
Function Dependency Validation:

Ensure all dependent functions exist and meet project requirements before modifying or committing changes.
Test Updates:

Always update tests to cover new functionality or edge cases introduced during module modifications.
References
Rust Documentation
Cargo Command Cheatsheet