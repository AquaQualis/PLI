fn tokenize_pli(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;

    let mut chars = text.chars().peekable(); // Allows peeking at the next character

    while let Some(c) = chars.next() {
        if in_string {
            // Handle string literals
            current_token.push(c);
            if c == '\'' {
                in_string = false; // End of string literal
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else if c == '\'' {
            // Start of string literal
            in_string = true;
            current_token.push(c);
        } else if c == '%' {
            // Handle % followed by a keyword
            current_token.push(c);
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() {
                    current_token.push(next_c);
                    chars.next(); // Consume the character
                } else {
                    break;
                }
            }
            tokens.push(current_token.clone());
            current_token.clear();
        } else if c.is_whitespace() {
            // Handle whitespace as a delimiter
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else if "!@#$%^&*()-+=[]{}|\\:;,.<>?/".contains(c) {
            // Special characters are standalone tokens
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            tokens.push(c.to_string());
        } else {
            // Add valid characters to the current token
            current_token.push(c);
        }
    }

    // Push the last token if not empty
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pli_directives() {
        let input = "%IF MAX > 10 %THEN;";
        let expected = vec!["%IF", "MAX", ">", "10", "%THEN", ";"];
        assert_eq!(tokenize_pli(input), expected);
    }

    #[test]
    fn test_string_literals() {
        let input = "SET A = 'Hello, World!';";
        let expected = vec!["SET", "A", "=", "'Hello, World!'", ";"];
        assert_eq!(tokenize_pli(input), expected);
    }

    #[test]
    fn test_comments() {
        let input = "%COMMENT This is a comment;";
        let expected = vec!["%COMMENT", "This", "is", "a", "comment", ";"];
        assert_eq!(tokenize_pli(input), expected);
    }

    #[test]
    fn test_mixed_input() {
        let input = "%DO I = 1 TO 5; SET TOTAL = TOTAL + I;";
        let expected = vec!["%DO", "I", "=", "1", "TO", "5", ";", "SET", "TOTAL", "=", "TOTAL", "+", "I", ";"];
        assert_eq!(tokenize_pli(input), expected);
    }
}

fn main() {
    let input = "%IF MAX > 10 %THEN; SET A = 'Hello, World!'; %COMMENT End of program.";
    let tokens = tokenize_pli(input);
    println!("Tokens: {:?}", tokens);
}

