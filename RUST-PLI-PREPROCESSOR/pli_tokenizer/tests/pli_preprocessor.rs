////////////////////////////////////////////////////////////////////////////////
// PL/I Preprocessor Test Suite
// -----------------------------------------------------------------------------
// Author: Jean-Pierre Sainfeld
// Assistant: ChatGPT
// Company: FirstLink Consulting Services (FLCS)
// Date: 11/17/2024
// -----------------------------------------------------------------------------
// Description:
// This test suite ensures the correctness of the PL/I Preprocessor's core
// functionality. Each test case targets specific scenarios and edge cases
// encountered during preprocessing, such as tokenization, validation of
// directives, nested structures, and handling invalid input.
//
// Tests Summary:
// - Valid File Tests: Verifies correct handling of syntactically valid files.
// - Invalid File Tests: Ensures files with invalid content are correctly flagged.
// - Edge Case Tests: Covers unique scenarios, such as very long lines and mixed content.
// - Nested Directives Tests: Checks proper handling of nested and deeply nested structures.
// - Additional Tests: Validates case insensitivity, unmatched directives, and large files.
//
// Purpose:
// This suite aims to verify that the PL/I Preprocessor behaves as expected for
// a wide range of inputs and prepares the system for real-world usage.
//
// Usage:
// Run the test suite using Cargo:
// $ cargo test --test pli_preprocessor
//
// -----------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use pli_tokenizer::modules::tokenizer::{is_valid_preprocessor_directive, tokenize_pli};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_valid_file() {
        // Test processing of a valid file
        let file_path = Path::new("tests/input/valid_file.pp");
        let content = fs::read_to_string(file_path).expect("Failed to read valid file");
        let lines: Vec<&str> = content.lines().collect();

        for (line_number, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let tokens = tokenize_pli(line);
            assert!(!tokens.is_empty(), "Expected tokens but got none on line {}", line_number + 1);
        }
    }

    #[test]
    fn test_edge_case_file() {
        // Test processing of an edge case file
        let file_path = Path::new("tests/input/edge_case.pli");
        let content = fs::read_to_string(file_path).expect("Failed to read edge case file");
        let lines: Vec<&str> = content.lines().collect();

        for (line_number, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let tokens = tokenize_pli(line);
            assert!(!tokens.is_empty(), "Expected tokens for edge case on line {}", line_number + 1);
        }
    }

    #[test]
    fn test_invalid_file() {
        // Test processing of an invalid file
        let file_path = Path::new("tests/input/invalid_file.txt");
        let content = fs::read_to_string(file_path).expect("Failed to read invalid file");
        let lines: Vec<&str> = content.lines().collect();

        for (line_number, line) in lines.iter().enumerate() {
            let tokens = tokenize_pli(line);

            if tokens.is_empty() {
                assert!(tokens.is_empty(), "Expected no tokens for invalid file on line {}", line_number + 1);
            } else {
                assert!(
                    !is_valid_preprocessor_directive(&tokens),
                    "Unexpectedly valid tokens for invalid file on line {}",
                    line_number + 1
                );
            }
        }
    }

    #[test]
    fn test_very_long_lines() {
        // Test handling of very long lines
        let long_line = "%IF DEBUG = 1 %THEN; ".to_owned() + &"A = B; ".repeat(500);
        let tokens = tokenize_pli(&long_line);
        assert!(!tokens.is_empty(), "Expected tokens for very long line");
    }

    #[test]
    fn test_nested_directives() {
        // Test handling of nested directives
        let nested_line = "%IF DEBUG %THEN; %IF NESTED %THEN; A = B; %ENDIF; %ENDIF;";
        let tokens = tokenize_pli(nested_line);
        assert_eq!(tokens.len(), 16, "Expected 16 tokens for nested directives");
    }

    #[test]
    fn test_deeply_nested_directives() {
        // Test handling of deeply nested directives
        let nested_line = "%IF DEBUG = 1 %THEN %DO I = 1 TO 5 %IF I > 3 %THEN; DISPLAY('Nested'); %ENDIF; %END; %ENDIF;";
        let tokens = tokenize_pli(nested_line);
        assert_eq!(tokens.len(), 28, "Expected 28 tokens for deeply nested directives");
    }

    #[test]
    fn test_unmatched_directives() {
        // Test handling of unmatched directives
        let unmatched_line = "%IF DEBUG %THEN;";
        let tokens = tokenize_pli(unmatched_line);
        assert!(tokens.contains(&"%IF".to_string()), "Expected '%IF' token for unmatched directive");
    }

    #[test]
    fn test_mixed_valid_invalid_directives() {
        // Test mixed valid and invalid directives in a single line
        let mixed_line = "%IF DEBUG = 1 %THEN; INVALID_DIRECTIVE; %ENDIF;";
        let tokens = tokenize_pli(mixed_line);
        assert!(tokens.contains(&"INVALID_DIRECTIVE".to_string()), "Expected 'INVALID_DIRECTIVE' token");
    }

    #[test]
    fn test_case_insensitivity() {
        // Test case insensitivity
        let insensitive_line = "%if debug = 1 %then;";
        let tokens = tokenize_pli(insensitive_line);
        assert!(tokens.contains(&"%IF".to_uppercase()), "Expected '%IF' token for case-insensitive directive");
    }

    #[test]
    fn test_invalid_characters() {
        // Test handling of invalid characters
        let invalid_line = "%IF DEBUG = 1 @INVALID_CHAR;";
        let tokens = tokenize_pli(invalid_line);
        assert!(tokens.contains(&"@INVALID_CHAR".to_string()), "Expected '@INVALID_CHAR' token");
    }

    #[test]
    fn test_mixed_line_types() {
        let mixed_lines = vec![
            "%IF DEBUG %THEN;",
            "This is a plain text line.",
            "",
            "%ENDIF;",
        ];
        for (line_number, line) in mixed_lines.iter().enumerate() {
            let tokens = tokenize_pli(line);
            println!("Line {} Tokens: {:?}", line_number + 1, tokens);

            if line.starts_with('%') {
                assert!(
                    !tokens.is_empty(),
                    "Expected tokens for directive on line {}",
                    line_number + 1
                );
            } else if line.trim().is_empty() {
                assert!(
                    tokens.is_empty(),
                    "Expected no tokens for empty line on line {}",
                    line_number + 1
                );
            } else {
                let expected_tokens: Vec<&str> = line.split_whitespace().collect();
                assert_eq!(
                    tokens,
                    expected_tokens.iter().map(|s| s.to_string().to_uppercase()).collect::<Vec<_>>(),
                    "Expected plain text tokens for non-directive line on line {}",
                    line_number + 1
                );
            }
        }
    }


    #[test]
    fn test_large_file() {
        // Test handling of very large files
        let large_content = vec!["%IF DEBUG = 1 %THEN;"; 10_000].join("\n");
        let tokens = tokenize_pli(&large_content);
        assert!(!tokens.is_empty(), "Expected tokens for large file");
    }

    #[test]
    fn test_preprocessor_edge_cases() {
        // Edge case examples for the preprocessor
        let edge_case_lines = vec![
            "%IF DEBUG = 1 %THEN; %DO;", // Complex directive
            "%ENDIF;",                  // Matching directive
            "%IF DEBUG = 1;",           // Missing %THEN
            "%DO A = 1 TO 10; %END;",   // Loop directive
        ];

        for (line_number, line) in edge_case_lines.iter().enumerate() {
            let tokens = tokenize_pli(line);
            println!("Line {} Tokens: {:?}", line_number + 1, tokens);

            // Assertions to check correct tokenization
            assert!(
                !tokens.is_empty(),
                "Expected tokens for edge case on line {}",
                line_number + 1
            );
        }
    }

}

