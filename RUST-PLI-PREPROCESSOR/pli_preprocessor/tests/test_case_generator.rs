use rand::{distributions::Alphanumeric, Rng};

/// Generates a variety of test cases for the tokenizer
pub fn generate_test_cases() -> Vec<String> {
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
