/*!
 * @file mod.rs
 * @brief Entry point for the tokenizer module.
 * 
 * This module orchestrates the submodules for tokenizing PL/I preprocessor input.
 * It re-exports the functionality provided by submodules such as `token`, `directive`,
 * `string_literal`, `special_char`, and `utils` to simplify access.
 * 
 * @author
 * - Jean-Pierre Sainfeld
 * - Assistant: ChatGPT
 * 
 * @company FirstLink Consulting Services (FLCS)
 * 
 * @version 1.0
 * @date 2024-11-24
 * 
 * @see token.rs
 * @see directive.rs
 * @see string_literal.rs
 * @see special_char.rs
 * @see utils.rs
 * @see tokenizer_logic.rs
 */

 pub mod directive;
 pub mod special_char;
 pub mod string_literal;
 pub mod token;
 pub mod utils;
 pub mod tokenizer_logic;

 
 // Explicitly re-export specific items to avoid ambiguity.
 pub use directive::get_directive_category;
 pub use token::{Token, TokenCategory};
 pub use utils::{to_uppercase, join_with_delimiter, is_blank, split_preserving_quotes};
 pub use tokenizer_logic::{tokenize_pli, has_tokenizer_error, is_valid_preprocessor_directive};
