////////////////////////////////////////////////////////////////////////////////
// TEST MODULE: Evaluator Tests
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This test module validates the functionality of the `evaluator` module.
//
// TEST CASES:
// - Test basic expression evaluation.
// - Test invalid expressions.
// - Test unsupported operators and edge cases.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use pli_tokenizer::modules::evaluator::{
        evaluate_expression, evaluate_operator, parse_and_evaluate, tokenize_expression,
    };

    #[test]
    fn test_evaluate_expression_simple() {
        assert_eq!(evaluate_expression("3 + 5"), Ok(8));
        assert_eq!(evaluate_expression("10 - 4"), Ok(6));
    }

    #[test]
    fn test_evaluate_expression_invalid() {
        assert!(evaluate_expression("").is_err());
        assert!(evaluate_expression("3 / 0").is_err());
        assert!(evaluate_expression("3 +").is_err());
    }

    #[test]
    fn test_tokenize_expression() {
        assert_eq!(
            tokenize_expression("3 + 5"),
            Ok(vec!["3".to_string(), "+".to_string(), "5".to_string()])
        );
    }

    #[test]
    fn test_parse_and_evaluate() {
        let tokens = vec!["3".to_string(), "+".to_string(), "5".to_string()];
        assert_eq!(parse_and_evaluate(&tokens), Ok(8));
    }

    #[test]
    fn test_evaluate_operator() {
        assert_eq!(evaluate_operator(3, 5, "+"), Ok(8));
        assert_eq!(evaluate_operator(10, 2, "/"), Ok(5));
        assert!(evaluate_operator(3, 0, "/").is_err());
        assert!(evaluate_operator(3, 5, "%").is_err());
    }

    #[test]
    fn test_evaluate_expression_complex() {
        assert_eq!(evaluate_expression("10 + 2 * 3"), Ok(16));
    }

    #[test]
    fn test_evaluate_expression_with_whitespace() {
        assert_eq!(evaluate_expression("  3   +    4 "), Ok(7));
    }

    #[test]
    fn test_evaluate_expression_unsupported_operator() {
        assert!(evaluate_expression("3 ^ 5").is_err());
    }
}
