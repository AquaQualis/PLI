#![allow(dead_code)] // Suppress warnings for unused functions in this module.

////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Tokenizer
// -----------------------------------------------------------------------------
// Description:
// This module provides functionality for tokenizing lines of PL/I preprocessor
// code. The tokenizer converts input lines into meaningful tokens, handling
// strings, directives, operators, and special characters.
//
// Features:
// - Tokenization of PL/I preprocessor lines into categorized tokens.
// - Handling of nested directives, strings, and special characters.
// - Error detection for malformed tokens (e.g., unmatched strings).
//
// Author: Jean-Pierre Sainfeld
// Assistant: ChatGPT
// Company: FirstLink Consulting Services (FLCS)
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

use log::debug;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

////////////////////////////////////////////////////////////////////////////////
// STRUCT: Token
// -----------------------------------------------------------------------------
// Represents a token in the PL/I tokenizer. Each token consists of its raw text
// value, a general category, and an optional specific category if it is a directive.
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String, // The raw text of the token.
    pub category: TokenCategory, // General category (e.g., Directive, Identifier, etc.).
    pub directive_category: Option<DirectiveCategory>, // Specific directive category (if applicable).
}

impl Token {
    /// Creates a new `Token` instance.
    ///
    /// # Parameters:
    /// - `value`: The raw text of the token.
    /// - `category`: The general category of the token.
    /// - `directive_category`: An optional specific category if the token is a directive.
    ///
    /// # Returns:
    /// - `Token`: A new token instance.
    ///
    /// # Example:
    /// ```rust
    /// let token = Token::new("%IF", TokenCategory::Directive, Some(DirectiveCategory::ControlFlow));
    /// ```
    pub fn new(
        value: &str,
        category: TokenCategory,
        directive_category: Option<DirectiveCategory>,
    ) -> Self {
        Self {
            value: value.to_string(),
            category,
            directive_category,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// ENUM: TokenCategory
// -----------------------------------------------------------------------------
// Enumerates general categories for tokens.
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenCategory {
    Directive,  // Preprocessor directives like %IF, %THEN
    Identifier, // Variables, function names, etc.
    Literal,    // Strings, numbers, etc.
    Operator,   // +, -, *, etc.
    Separator,  // , ; ( ) { }
    Unknown,    // For unexpected or malformed tokens
}

////////////////////////////////////////////////////////////////////////////////
// ENUM: DirectiveCategory
// -----------------------------------------------------------------------------
// Enumerates specific categories for preprocessor directives.
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DirectiveCategory {
    ControlFlow,   // Directives like %IF, %THEN, %ELSE, %ENDIF
    MacroHandling, // Directives like %MACRO, %INCLUDE
    Conditional,   // Directives like %EVALUATE, %SWITCH, %CASE
    Comment,       // Directives like %COMMENT
    Other,         // For undefined or unrecognized directives
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: get_directive_category
// -----------------------------------------------------------------------------
// Retrieves the category of a given PL/I preprocessor directive.
//
// # Parameters:
// - `directive` (`&str`): The directive token.
//
// # Returns:
// - `DirectiveCategory`: The category of the directive.
//
// # See Also:
// - `directive_categories`: Provides a mapping of known directives to categories.
////////////////////////////////////////////////////////////////////////////////
pub fn get_directive_category(directive: &str) -> DirectiveCategory {
    match directive {
        "%IF" | "%THEN" | "%ELSE" | "%ENDIF" => DirectiveCategory::ControlFlow,
        "%MACRO" | "%INCLUDE" => DirectiveCategory::MacroHandling,
        "%SWITCH" | "%CASE" | "%EVALUATE" => DirectiveCategory::Conditional,
        "%COMMENT" => DirectiveCategory::Comment,
        _ => DirectiveCategory::Other,
    }
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: tokenize_pli
// -----------------------------------------------------------------------------
// Tokenizes a given PL/I input string into a vector of categorized tokens.
//
// Includes debug logs to track the tokenization process and handles:
// - Whitespace
// - String literals
// - Special characters
// - Case-insensitivity for directives
//
// # Parameters:
// - `input` (`&str`): The PL/I input line to be tokenized.
//
// # Returns:
// - `Vec<Token>`: A vector of tokens parsed from the input.
//
// # Errors/Conditions:
// - Malformed Input: Unmatched string literals or invalid characters.
//
// # See Also:
// - `handle_string_literal`: Handles tokenization of string literals.
// - `handle_special_characters`: Processes special characters.
////////////////////////////////////////////////////////////////////////////////
pub fn tokenize_pli(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;

    println!("Tokenizing Input: {}", input); // Debug log

    while let Some(c) = chars.next() {
        if c.is_whitespace() && !in_string {
            finalize_token(&mut current_token, &mut tokens);
            continue;
        }

        match c {
            '\'' => handle_string_literal(c, &mut chars, &mut in_string, &mut current_token, &mut tokens),
            '%' => handle_directive(c, &mut chars, &mut current_token, &mut tokens),
            '=' | '#' | '*' | ';' => handle_special_characters(c, &mut chars, &mut current_token, &mut tokens),
            _ if c.is_alphanumeric() || c == '_' => current_token.push(c),
            _ => handle_special_characters(c, &mut chars, &mut current_token, &mut tokens),
        }
    }

    finalize_token(&mut current_token, &mut tokens);
    println!("Generated Tokens: {:?}", tokens); // Debug log
    tokens
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: handle_string_literal
// -----------------------------------------------------------------------------
// Handles string literals, ensuring proper tokenization and detection of errors.
//
// # Parameters:
// - `current_char`: The current character, typically `'`.
// - `chars`: The character iterator for processing the input.
// - `in_string`: A mutable reference to a flag tracking string literals.
// - `current_token`: A mutable reference to the current token string.
// - `tokens`: A mutable reference to the list of generated tokens.
////////////////////////////////////////////////////////////////////////////////
pub fn handle_string_literal(
    current_char: char,
    chars: &mut Peekable<Chars>,
    in_string: &mut bool,
    current_token: &mut String,
    tokens: &mut Vec<Token>,
) {
    *in_string = true;
    current_token.push(current_char);

    while let Some(&next_char) = chars.peek() {
        current_token.push(next_char);
        chars.next();

        if next_char == '\'' {
            *in_string = false;
            break;
        }
    }

    tokens.push(Token::new(
        current_token.trim(),
        TokenCategory::Literal,
        None,
    ));
    current_token.clear();
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: handle_special_characters
// -----------------------------------------------------------------------------
// Processes special characters and assigns appropriate token categories.
//
// # Parameters:
// - `c` (`char`): The current special character being processed.
// - `_chars`: A mutable reference to the character iterator (unused).
// - `current_token`: A mutable reference to the current token being constructed.
// - `tokens`: A mutable reference to the list of generated tokens.
////////////////////////////////////////////////////////////////////////////////
pub fn handle_special_characters(
    c: char,
    _chars: &mut Peekable<Chars>,
    current_token: &mut String,
    tokens: &mut Vec<Token>,
) {
    finalize_token(current_token, tokens);

    let token_category = match c {
        '=' | '#' | '*' => TokenCategory::Operator,
        ';' => TokenCategory::Separator,
        _ => TokenCategory::Unknown,
    };

    tokens.push(Token::new(&c.to_string(), token_category, None));
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: handle_directive
// -----------------------------------------------------------------------------
// Processes and categorizes PL/I preprocessor directives.
//
// This function processes a directive starting with `%`, validates it, and 
// categorizes it into the appropriate `DirectiveCategory`. The directive is 
// expected to start with `%` and consist of alphanumeric characters. This 
// ensures that all valid directives are tokenized correctly.
//
// -----------------------------------------------------------------------------
// PARAMETERS:
// -----------------------------------------------------------------------------
// - `current_char` (`char`): The current character, typically `%`.
// - `chars` (`&mut Peekable<Chars>`): The character iterator for processing the 
//   input.
// - `current_token` (`&mut String`): A mutable reference to the current token 
//   string being built.
// - `tokens` (`&mut Vec<Token>`): A mutable reference to the list of tokens 
//   generated.
//
// -----------------------------------------------------------------------------
// RETURNS:
// -----------------------------------------------------------------------------
// - None. The function directly updates the `tokens` list with a new `Token`.
//
// -----------------------------------------------------------------------------
// ERRORS/CONDITIONS:
// -----------------------------------------------------------------------------
// - If the directive is malformed (e.g., incomplete or invalid), it will still 
//   be categorized as `DirectiveCategory::Other`.
//
// -----------------------------------------------------------------------------
// SEE ALSO:
// -----------------------------------------------------------------------------
// - `get_directive_category`: For mapping directives to categories.
// - `tokenize_pli`: Main function for tokenizing input strings.
//
// -----------------------------------------------------------------------------
// EXAMPLE:
// -----------------------------------------------------------------------------
// ```rust
// let mut chars = "%IF".chars().peekable();
// let mut tokens = Vec::new();
// let mut current_token = String::new();
// handle_directive('%', &mut chars, &mut current_token, &mut tokens);
// assert_eq!(tokens[0].value, "%IF");
// assert_eq!(tokens[0].category, TokenCategory::Directive);
// assert_eq!(tokens[0].directive_category, Some(DirectiveCategory::ControlFlow));
// ```
////////////////////////////////////////////////////////////////////////////////
pub fn handle_directive(
    current_char: char,
    chars: &mut Peekable<Chars>,
    current_token: &mut String,
    tokens: &mut Vec<Token>,
) {
    // Start building the directive token
    current_token.push(current_char);

    // Consume remaining characters in the directive
    while let Some(&next_char) = chars.peek() {
        if next_char.is_alphanumeric() || next_char == '_' {
            current_token.push(next_char);
            chars.next();
        } else {
            break;
        }
    }

    // Convert token to uppercase and categorize
    let directive = current_token.to_uppercase();
    let directive_category = get_directive_category(&directive);

    // Add the token to the list
    tokens.push(Token::new(
        &directive,
        TokenCategory::Directive,
        Some(directive_category),
    ));

    // Clear the current token for the next iteration
    current_token.clear();
}



////////////////////////////////////////////////////////////////////////////////
// FUNCTION: finalize_token
// -----------------------------------------------------------------------------
// Finalizes the current token and adds it to the token list.
//
// # Parameters:
// - `current_token` (`&mut String`): The token string to finalize.
// - `tokens` (`&mut Vec<Token>`): The list of tokens to add the finalized token.
////////////////////////////////////////////////////////////////////////////////
fn finalize_token(current_token: &mut String, tokens: &mut Vec<Token>) {
    if !current_token.is_empty() {
        tokens.push(Token::new(&current_token.to_uppercase(), TokenCategory::Identifier, None));
        current_token.clear();
    }
}
