////////////////////////////////////////////////////////////////////////////////
// PL/I Preprocessor Library
// -----------------------------------------------------------------------------
// Author: Jean-Pierre Sainfeld
// Assistant: ChatGPT
// Company: FirstLink Consulting Services (FLCS)
// Date: 11/24/2024
// -----------------------------------------------------------------------------
// Description:
// This library implements the core functionality of the PL/I Preprocessor.
// It provides modular components for tokenization, validation, macro expansion,
// conditional evaluation, include resolution, and more.
//
// Features:
// - **Tokenization**: Converts lines of PL/I code into meaningful tokens.
// - **Validation**: Validates syntax and preprocessor directives.
// - **Macro Expansion**: Resolves and expands macros in PL/I source code.
// - **Conditional Evaluation**: Supports conditional execution of code blocks.
// - **Include Resolution**: Handles include directives to integrate external files.
// - **Logging**: Provides a detailed logging mechanism for debugging and diagnostics.
//
// Modular Design:
// The library is divided into distinct modules, each handling a specific aspect
// of the preprocessing workflow. This approach promotes maintainability and
// reusability across different projects.
//
// Purpose:
// The library serves as the backbone of the PL/I Preprocessor project,
// facilitating efficient and accurate transformation of PL/I source code.
//
// Usage:
// This library is used in conjunction with the `main.rs` program, which
// orchestrates the overall preprocessing workflow. The modular design allows
// individual components to be unit-tested and extended independently.
//
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

#![allow(unused_imports)] // Allows unused imports during development.

pub mod modules {
    pub mod conditional;
    pub mod evaluator;
    pub mod include_handler;
    pub mod logger;
    pub mod macro_expander;
    pub mod output;
    pub mod parser;
    pub mod tokenizer;
    pub mod validator;
}
