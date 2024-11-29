fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for c in text.chars() {
        // Check if the character is part of a token
        if c.is_alphanumeric() || c == '#' || c == '\'' || c == '-' || c == '@' || c == '.' {
            current_token.push(c);
        } else {
            // Push the current token when encountering a delimiter
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        }
    }

    // Push the last token if not empty
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    // Final filtering and cleanup
    tokens.into_iter()
        .map(|token| token.trim_end_matches('.').to_string()) // Trim trailing periods
        .filter(|token| {
            // Validate email tokens
            if token.contains('@') && token.contains('.') {
                let parts: Vec<&str> = token.split('@').collect();
                if parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.') {
                    return true;
                }
            }
            // Keep valid tokens and avoid empty ones
            !token.is_empty() && !token.starts_with('.') && !token.ends_with('.')
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyphenated_words() {
        let input = "state-of-the-art design";
        let expected = vec!["state-of-the-art", "design"];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_contractions() {
        let input = "Don't stop believing!";
        let expected = vec!["Don't", "stop", "believing"];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_hashtags() {
        let input = "#RustLang is amazing!";
        let expected = vec!["#RustLang", "is", "amazing"];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_emails() {
        let input = "Contact us at info@example.com.";
        let expected = vec!["Contact", "us", "at", "info@example.com"];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_mixed_input() {
        let input = "Contact info@example.com #RustLang Don't panic!";
        let expected = vec!["Contact", "info@example.com", "#RustLang", "Don't", "panic"];
        assert_eq!(tokenize(input), expected);
    }
}

fn main() {
    let input = "Contact us at info@example.com or #RustLang is amazing!";
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);
}

