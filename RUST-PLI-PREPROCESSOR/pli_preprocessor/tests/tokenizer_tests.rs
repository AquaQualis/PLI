/*!
 * @file tokenizer_tests.rs
 * @brief Unit tests for the PL/I tokenizer module.
 *
 * @details
 * This file contains unit tests for the `tokenizer` module, which is responsible
 * for tokenizing lines of PL/I preprocessor code into structured tokens. Each
 * test validates specific aspects of the tokenization process, ensuring robust
 * behavior under various conditions.
 *
 * @author
 * - Jean-Pierre Sainfeld
 * - Assistant: ChatGPT
 *
 * @company
 * FirstLink Consulting Services (FLCS)
 */

 #[cfg(test)]
 mod tests {
     use pli_preprocessor::modules::tokenizer::{tokenize_pli};
 
     /// @test test_basic_directives
     /// @brief Tests the tokenization of a basic directive sequence.
     ///
     /// @details
     /// This test validates that basic PL/I directives and their associated tokens
     /// are correctly identified and categorized.
     ///
     /// @input
     /// Input: `%IF DEBUG %THEN;`
     ///
     /// @expected
     /// Output: `["%IF", "DEBUG", "%THEN", ";"]`
     #[test]
     fn test_basic_directives() {
         let input = "%IF DEBUG %THEN;";
         let expected = vec!["%IF", "DEBUG", "%THEN", ";"];
         let tokens: Vec<String> = tokenize_pli(input).iter().map(|t| t.value.clone()).collect();
         assert_eq!(tokens, expected);
     }
 
     /// @test test_edge_case_incomplete_directive
     /// @brief Tests an incomplete directive for correct tokenization.
     ///
     /// @details
     /// This test ensures that even incomplete directives are properly tokenized
     /// without causing errors.
     ///
     /// @input
     /// Input: `%IF DEBUG`
     ///
     /// @expected
     /// Output: `["%IF", "DEBUG"]`
     #[test]
     fn test_edge_case_incomplete_directive() {
         let input = "%IF DEBUG";
         let expected = vec!["%IF", "DEBUG"];
         let tokens: Vec<String> = tokenize_pli(input).iter().map(|t| t.value.clone()).collect();
         assert_eq!(tokens, expected);
     }
 
     /// @test test_long_line
     /// @brief Tests tokenization of a long line with multiple tokens.
     ///
     /// @details
     /// This test checks the tokenizer's ability to handle a long line with
     /// various tokens, including strings and special characters.
     ///
     /// @input
     /// Input: `%IF DEBUG %THEN; SET A = 'Long line with multiple tokens and 1234567890'; %ENDIF;`
     ///
     /// @expected
     /// Output: `["%IF", "DEBUG", "%THEN", ";", "SET", "A", "=", "'Long line with multiple tokens and 1234567890'", ";", "%ENDIF", ";"]`
     #[test]
     fn test_long_line() {
         let input = "%IF DEBUG %THEN; SET A = 'Long line with multiple tokens and 1234567890'; %ENDIF;";
         let expected = vec![
             "%IF", "DEBUG", "%THEN", ";", "SET", "A", "=", "'Long line with multiple tokens and 1234567890'", ";", "%ENDIF", ";",
         ];
         let tokens: Vec<String> = tokenize_pli(input).iter().map(|t| t.value.clone()).collect();
         assert_eq!(tokens, expected);
     }
 
     /// @test test_mixed_content
     /// @brief Tests tokenization of mixed PL/I and preprocessor code.
     ///
     /// @details
     /// This test ensures the tokenizer can handle mixed lines containing PL/I
     /// code and preprocessor directives.
     ///
     /// @input
     /// Input: `SET A = 'Regular PL/I code' %IF DEBUG %THEN;`
     ///
     /// @expected
     /// Output: `["SET", "A", "=", "'Regular PL/I code'", "%IF", "DEBUG", "%THEN", ";"]`
     #[test]
     fn test_mixed_content() {
         let input = "SET A = 'Regular PL/I code' %IF DEBUG %THEN;";
         let expected = vec![
             "SET", "A", "=", "'Regular PL/I code'", "%IF", "DEBUG", "%THEN", ";",
         ];
         let tokens: Vec<String> = tokenize_pli(input).iter().map(|t| t.value.clone()).collect();
         assert_eq!(tokens, expected);
     }
 
     /// @test test_special_characters
     /// @brief Tests tokenization of lines with special characters.
     ///
     /// @details
     /// This test validates the tokenizer's ability to handle special characters
     /// and correctly assign them to tokens.
     ///
     /// @input
     /// Input: `%IF DEBUG *&^%$#@!(){}[]<>;`
     ///
     /// @expected
     /// Output: `["%IF", "DEBUG", "*", "&", "^", "%", "$", "#", "@", "!", "(", ")", "{", "}", "[", "]", "<", ">", ";"]`
     #[test]
     fn test_special_characters() {
         let input = "%IF DEBUG *&^%$#@!(){}[]<>;";
         let expected = vec![
             "%IF", "DEBUG", "*", "&", "^", "%", "$", "#", "@", "!", "(", ")", "{", "}", "[", "]", "<", ">", ";",
         ];
         let tokens: Vec<String> = tokenize_pli(input).iter().map(|t| t.value.clone()).collect();
         assert_eq!(tokens, expected);
     }
 
     /// @test test_empty_input
     /// @brief Tests tokenization of an empty input string.
     ///
     /// @details
     /// This test ensures that the tokenizer handles empty inputs gracefully
     /// without generating any tokens or errors.
     ///
     /// @input
     /// Input: `""`
     ///
     /// @expected
     /// Output: `[]`
     #[test]
     fn test_empty_input() {
         let input = "";
         let expected: Vec<String> = vec![];
         let tokens: Vec<String> = tokenize_pli(input).iter().map(|t| t.value.clone()).collect();
         assert_eq!(tokens, expected);
     }
 }
 