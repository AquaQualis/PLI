////////////////////////////////////////////////////////////////////////////////
// TESTS FOR: Output Handler
// ----------------------------------------------------------------------------
// These tests verify the functionality of the `output` module, ensuring proper
// handling of file operations such as writing and appending lines.
// ----------------------------------------------------------------------------
// AUTHOR: FirstLink Consulting Services (FLCS)
// DATE: 11/17/2024
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use pli_tokenizer::modules::output::{append_log_message, write_line_to_file};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_write_line_to_file() {
        let test_file = Path::new("/tmp/test_output.txt");
        let content = "Test line";

        // Write to file
        write_line_to_file(test_file, content).unwrap();

        // Verify file content
        let file_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(file_content, content);

        // Clean up
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_append_log_message() {
        let test_log = Path::new("/tmp/test_log.txt");
        let message1 = "First log entry";
        let message2 = "Second log entry";

        // Append messages
        append_log_message(test_log, message1).unwrap();
        append_log_message(test_log, message2).unwrap();

        // Verify log content
        let log_content = fs::read_to_string(test_log).unwrap();
        let expected_content = format!("{}\n{}\n", message1, message2);
        assert_eq!(log_content, expected_content);

        // Clean up
        fs::remove_file(test_log).unwrap();
    }
}
