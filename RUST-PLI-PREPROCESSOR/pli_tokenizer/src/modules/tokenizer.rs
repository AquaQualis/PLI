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
    /// # Arguments
    /// - `value`: The raw text of the token.
    /// - `category`: The general category of the token.
    /// - `directive_category`: An optional specific category if the token is a directive.
    ///
    /// # Returns
    /// A new `Token` instance.
    ///
    /// # Example
    /// ```rust
    /// let token = Token::new("%IF", TokenCategory::Directive, Some(DirectiveCategory::ControlFlow));
    /// ```
    pub fn new(value: &str, category: TokenCategory, directive_category: Option<DirectiveCategory>) -> Self {
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
    ControlFlow,       // Directives like %IF, %THEN, %ELSE, %ENDIF
    MacroHandling,     // Directives like %MACRO, %INCLUDE
    Conditional,       // Directives like %EVALUATE, %SWITCH, %CASE
    Comment,           // Directives like %COMMENT
    Other,             // For undefined or unrecognized directives
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: directive_categories
// -----------------------------------------------------------------------------
// Returns a mapping of PL/I preprocessor directives to their specific categories.
// -----------------------------------------------------------------------------
pub fn directive_categories() -> HashMap<&'static str, DirectiveCategory> {
    let mut categories = HashMap::new();

    // Control Flow
    categories.insert("%IF", DirectiveCategory::ControlFlow);
    categories.insert("%THEN", DirectiveCategory::ControlFlow);
    categories.insert("%ELSE", DirectiveCategory::ControlFlow);
    categories.insert("%ENDIF", DirectiveCategory::ControlFlow);

    // Macro Handling
    categories.insert("%MACRO", DirectiveCategory::MacroHandling);
    categories.insert("%INCLUDE", DirectiveCategory::MacroHandling);

    // Conditional
    categories.insert("%SWITCH", DirectiveCategory::Conditional);
    categories.insert("%CASE", DirectiveCategory::Conditional);
    categories.insert("%EVALUATE", DirectiveCategory::Conditional);

    // Comment
    categories.insert("%COMMENT", DirectiveCategory::Comment);

    categories
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: get_directive_category
// -----------------------------------------------------------------------------
// Retrieves the category of a given PL/I preprocessor directive.
// -----------------------------------------------------------------------------
pub fn get_directive_category(directive: &str) -> DirectiveCategory {
    directive_categories()
        .get(directive)
        .cloned()
        .unwrap_or(DirectiveCategory::Other)
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: tokenize_pli
// -----------------------------------------------------------------------------
// Tokenizes a given PL/I input string into a vector of categorized tokens.
// -----------------------------------------------------------------------------
pub fn tokenize_pli(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;

    println!("Tokenizing Input: {}", input); // Debug log

    while let Some(c) = chars.next() {
        if c.is_whitespace() && !in_string {
            if !current_token.is_empty() {
                println!("Whitespace found, token finalized: {}", current_token); // Debug log
                tokens.push(Token::new(
                    &current_token,
                    TokenCategory::Identifier,
                    None,
                ));
                current_token.clear();
            }
            continue;
        }

        if c == '\'' {
            handle_string_literal(c, &mut chars, &mut in_string, &mut current_token, &mut tokens);
            continue;
        }

        if c.is_alphanumeric() || c == '_' {
            current_token.push(c);
            continue;
        }

        handle_special_characters(c, &mut chars, &mut current_token, &mut tokens);
    }

    if !current_token.is_empty() {
        println!("Finalizing last token: {}", current_token); // Debug log
        tokens.push(Token::new(&current_token, TokenCategory::Identifier, None));
    }

    println!("Generated Tokens: {:?}", tokens); // Debug log
    tokens
}

////////////////////////////////////////////////////////////////////////////////
// FUNCTION: handle_string_literal
// -----------------------------------------------------------------------------
// Handles string literals in the input, ensuring proper tokenization.
// -----------------------------------------------------------------------------
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
// Processes special characters like `;`, `#`, `=` into appropriate tokens.
// -----------------------------------------------------------------------------
pub fn handle_special_characters(
    c: char,
    _chars: &mut Peekable<Chars>,
    current_token: &mut String,
    tokens: &mut Vec<Token>,
) {
    if !current_token.is_empty() {
        println!("Token finalized before special char: {}", current_token); // Debug log
        tokens.push(Token::new(
            current_token,
            TokenCategory::Identifier,
            None,
        ));
        current_token.clear();
    }

    let token_category = match c {
        '=' | ';' | '#' | '*' => TokenCategory::Operator,
        _ => TokenCategory::Unknown,
    };

    tokens.push(Token::new(&c.to_string(), token_category, None));
}
