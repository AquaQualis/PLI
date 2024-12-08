////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Validator Tests
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This test module verifies the functionality of the syntax validator module.
//
// TESTS INCLUDED:
// - Basic validations for valid and invalid syntax.
// - Edge case testing for unmatched directives.
// - Comprehensive testing of valid and invalid directives.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use pli_preprocessor::modules::validator::{is_valid_directive, validate_syntax};

    #[test]
    fn test_validate_syntax_basic() {
        let tokens = vec![
            "%IF".to_string(),
            "DEBUG".to_string(),
            "%THEN".to_string(),
            "%ENDIF".to_string(),
        ];
        let result = validate_syntax(&tokens);
        assert!(result.is_ok(), "Basic syntax validation failed.");
    }

    #[test]
    fn test_validate_syntax_with_errors() {
        let tokens = vec!["%IF".to_string(), "DEBUG".to_string(), "%THEN".to_string()];
        let result = validate_syntax(&tokens);
        assert!(
            result.is_err(),
            "Validation did not detect missing %ENDIF for input: {:?}",
            tokens
        );
        assert_eq!(result.unwrap_err(), "Unmatched %IF found");
    }

    #[test]
    fn test_validate_syntax_edge_cases() {
        let tokens = vec![
            "%IF".to_string(),
            "%ENDIF".to_string(),
            "%ENDIF".to_string(),
        ];
        let result = validate_syntax(&tokens);
        assert!(
            result.is_err(),
            "Validation did not detect extra %ENDIF for input: {:?}",
            tokens
        );
        assert_eq!(result.unwrap_err(), "Unmatched %ENDIF found");
    }

    #[test]
    fn test_validate_syntax_nested() {
        let tokens = vec![
            "%IF".to_string(),
            "DEBUG".to_string(),
            "%THEN".to_string(),
            "%IF".to_string(),
            "NESTED".to_string(),
            "%THEN".to_string(),
            "%ENDIF".to_string(),
            "%ENDIF".to_string(),
        ];
        let result = validate_syntax(&tokens);
        assert!(result.is_ok(), "Nested syntax validation failed.");
    }

    #[test]
    fn test_is_valid_directive() {
        assert!(is_valid_directive("%IF"));
        assert!(!is_valid_directive("%INVALID"));
    }
}
