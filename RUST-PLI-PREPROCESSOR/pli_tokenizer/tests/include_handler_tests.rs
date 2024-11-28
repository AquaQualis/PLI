////////////////////////////////////////////////////////////////////////////////
// MODULE NAME: Include Handler Tests
// ----------------------------------------------------------------------------
// DESCRIPTION:
// This file contains unit tests for the `include_handler` module. It ensures
// the functionality of `%INCLUDE` directive processing in the PL/I preprocessor.
//
// AUTHOR: FirstLink Consulting Services (FLCS)
// LICENSE: MIT License
// DATE: 11/17/2024
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// IMPORTS
////////////////////////////////////////////////////////////////////////////////

use pli_tokenizer::modules::include_handler::*;
use std::fs;
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////
// TESTS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_file_path() {
        // Valid directive
        assert_eq!(
            extract_file_path("%INCLUDE 'example.pli';"),
            Some("example.pli".to_string())
        );

        // Missing file path
        assert_eq!(extract_file_path("%INCLUDE ;"), None);

        // Invalid directive
        assert_eq!(extract_file_path("INVALID"), None);

        // Missing quotes
        assert_eq!(
            extract_file_path("%INCLUDE example.pli;"),
            Some("example.pli".to_string())
        );

        // Empty directive
        assert_eq!(extract_file_path("%INCLUDE '';"), None);
    }

    #[test]
    fn test_resolve_include_path() {
        let current_dir = Path::new("/path/to/current");
        assert_eq!(
            resolve_include_path("example.pli", current_dir),
            Ok(Path::new("/path/to/current/example.pli").to_path_buf())
        );
        assert_eq!(
            resolve_include_path("/absolute/path/example.pli", current_dir),
            Ok(Path::new("/absolute/path/example.pli").to_path_buf())
        );
    }

    #[test]
    fn test_read_file() {
        let temp_file = "/tmp/test.pli";
        fs::write(temp_file, "Test content").unwrap();
        let content = read_file(Path::new(temp_file));
        assert_eq!(content.unwrap(), "Test content");
        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_process_include() {
        let current_dir = Path::new("/tmp");
        let temp_file = "/tmp/example.pli";
        fs::write(temp_file, "Test content").unwrap();
        let directive = "%INCLUDE 'example.pli';";
        let content = process_include(directive, current_dir);
        assert_eq!(content.unwrap(), "Test content");
        fs::remove_file(temp_file).unwrap();
    }
}
