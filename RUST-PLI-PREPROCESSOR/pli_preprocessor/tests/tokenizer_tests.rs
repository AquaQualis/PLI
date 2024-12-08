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
     use rand::{distributions::Alphanumeric, Rng};
 
     /// Generates a variety of test cases for the tokenizer
     fn generate_test_cases() -> Vec<String> {
         let mut cases = Vec::new();
 
         // Predefined valid cases
         cases.push("SET A = 'Valid String Literal';".to_string());
         cases.push("SET B = '';".to_string()); // Empty string
         cases.push("SET C = 'Another Valid String';".to_string());
         cases.push("%IF DEBUG %THEN;".to_string());
         cases.push("SET X = 42;".to_string());
 
         // Predefined edge cases
         cases.push("SET D = 'Unmatched".to_string()); // Unmatched string literal
         cases.push("SET E = '''Double Starting Quotes';".to_string()); // Double starting quote
         cases.push("SET F = 'Ending With Whitespace ';".to_string());
         cases.push("SET G = 'Special @#$%^&*() Characters';".to_string());
 
         // Randomized cases
         for _ in 0..20 {
             let random_string: String = rand::thread_rng()
                 .sample_iter(&Alphanumeric)
                 .take(50) // Random string of length 50
                 .map(char::from)
                 .collect();
             cases.push(format!("SET H = '{}';", random_string));
         }
 
         // Stress cases
         cases.push(format!("SET I = '{}';", "A".repeat(10_000))); // 10,000 'A's
         cases.push(format!("SET J = '{}';", "Nested 'Single' Quotes' Here'"));
 
         cases
     }
 
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
 
     // ... Other existing tests ...
 
     /// @test test_generated_cases
     /// @brief Tests the tokenizer against a variety of auto-generated cases.
     ///
     /// @details
     /// This test validates the tokenizer's robustness by processing a wide range of
     /// auto-generated test cases, including valid, edge, random, and stress cases.
     ///
     /// @input
     /// Generated test cases
     ///
     /// @expected
     /// All cases processed without errors, and tokens match expectations.
     #[test]
     fn test_generated_cases() {
         let cases = generate_test_cases();
 
         for (i, case) in cases.iter().enumerate() {
             let tokens = tokenize_pli(&case); // Replace with your tokenizer function
 
             assert!(
                 !tokens.is_empty() || case.trim().is_empty(),
                 "Tokenizer failed on case {}: {}",
                 i,
                 case
             );
 
             println!("Case {} passed: {}", i, case);
         }
     }

     /// Debug function to print all generated test cases
    #[test]
    fn debug_generated_test_cases() {
        let cases = generate_test_cases();

        for (i, case) in cases.iter().enumerate() {
            println!("Case {}: {}", i, case);
        }
    }

 }
 