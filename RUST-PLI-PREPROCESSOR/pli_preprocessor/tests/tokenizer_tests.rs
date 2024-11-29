#[cfg(test)]
mod tests {
    use pli_tokenizer::modules::tokenizer::tokenize_pli;

    #[test]
    fn test_basic_directives() {
        let input = "%IF DEBUG %THEN;";
        let expected = vec!["%IF", "DEBUG", "%THEN", ";"];
        assert_eq!(tokenize_pli(input), expected);
    }

    #[test]
    fn test_edge_case_incomplete_directive() {
        let input = "%IF DEBUG";
        let expected = vec!["%IF", "DEBUG"];
        assert_eq!(tokenize_pli(input), expected);
    }

    #[test]
    fn test_long_line() {
        let input =
            "%IF DEBUG %THEN; SET A = 'Long line with multiple tokens and 1234567890'; %ENDIF;";
        let expected = vec![
            "%IF",
            "DEBUG",
            "%THEN",
            ";",
            "SET",
            "A",
            "=",
            "'Long line with multiple tokens and 1234567890'",
            ";",
            "%ENDIF",
            ";",
        ];
        assert_eq!(tokenize_pli(input), expected);
    }

    #[test]
    fn test_mixed_content() {
        let input = "SET A = 'Regular PL/I code' %IF DEBUG %THEN;";
        let expected = vec![
            "SET",
            "A",
            "=",
            "'Regular PL/I code'",
            "%IF",
            "DEBUG",
            "%THEN",
            ";",
        ];
        assert_eq!(tokenize_pli(input), expected);
    }

    #[test]
    fn test_special_characters() {
        let input = "%IF DEBUG *&^%$#@!(){}[]<>;";
        let expected = vec![
            "%IF", "DEBUG", "*", "&", "^", "%", "$", "#", "@", "!", "(", ")", "{", "}", "[", "]",
            "<", ">", ";",
        ];
        assert_eq!(tokenize_pli(input), expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let expected: Vec<String> = vec![];
        assert_eq!(tokenize_pli(input), expected);
    }
}
