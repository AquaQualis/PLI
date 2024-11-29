////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Conditional Tests
// ----------------------------------------------------------------------------
// DESCRIPTION:
// Tests for the `Conditional Processor` module.
//
// FUNCTIONALITY:
// - Tests `process_condition` for various scenarios.
// - Validates nested conditional block structures using
//   `validate_conditional_structure`.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
// VERSION: 1.0.0
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use pli_tokenizer::modules::conditional::{process_condition, validate_conditional_structure};

    #[test]
    fn test_process_condition_valid() {
        assert_eq!(process_condition("DEBUG = 1"), Ok(true));
        assert_eq!(process_condition("DEBUG != 0"), Ok(true));
    }

    #[test]
    fn test_process_condition_invalid_format() {
        assert!(process_condition("DEBUG =").is_err());
        assert!(process_condition("").is_err());
    }

    #[test]
    fn test_process_condition_unknown_variable() {
        assert!(process_condition("UNKNOWN = 1").is_err());
    }

    #[test]
    fn test_validate_conditional_structure_valid() {
        let tokens = vec!["%IF".to_string(), "%ENDIF".to_string()];
        assert!(validate_conditional_structure(&tokens).is_ok());
    }

    #[test]
    fn test_validate_conditional_structure_unmatched_if() {
        let tokens = vec!["%IF".to_string()];
        assert!(validate_conditional_structure(&tokens).is_err());
    }

    #[test]
    fn test_validate_conditional_structure_unmatched_endif() {
        let tokens = vec!["%ENDIF".to_string()];
        assert!(validate_conditional_structure(&tokens).is_err());
    }

    #[test]
    fn test_validate_conditional_structure_nested_valid() {
        let tokens = vec![
            "%IF".to_string(),
            "%IF".to_string(),
            "%ENDIF".to_string(),
            "%ENDIF".to_string(),
        ];
        assert!(validate_conditional_structure(&tokens).is_ok());
    }

    #[test]
    fn test_validate_conditional_structure_nested_invalid() {
        let tokens = vec!["%IF".to_string(), "%IF".to_string(), "%ENDIF".to_string()];
        assert!(validate_conditional_structure(&tokens).is_err());
    }
}
