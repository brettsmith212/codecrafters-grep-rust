use std::env;
use std::io;
use std::process;

fn extract_pattern_letters(pattern: &str) -> Option<Vec<char>> {
    if pattern.len() < 3 {
        return None;
    }

    let chars = pattern[1..pattern.len() - 1].chars();

    let letters: Vec<char> = chars.filter(|c| c.is_alphabetic()).collect();

    if letters.is_empty() {
        None
    } else {
        Some(letters)
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern == "\\d" {
        input_line.chars().any(|c| c.is_ascii_digit())
    } else if pattern == "\\w" {
        input_line.chars().any(|c| c.is_alphanumeric() || c == '_')
    } else if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        match extract_pattern_letters(pattern) {
            Some(letters) => {
                for letter in letters {
                    if input_line.contains(letter) {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_digit_pattern() {
        assert!(match_pattern("abc123", "\\d"));
        assert!(!match_pattern("abcdef", "\\d"));
    }

    #[test]
    fn test_match_word_pattern() {
        assert!(match_pattern("hello_world", "\\w"));
        assert!(match_pattern("test123", "\\w"));
        assert!(!match_pattern("", "\\w"));
    }

    #[test]
    fn test_match_single_character() {
        assert!(match_pattern("hello", "h"));
        assert!(match_pattern("world", "o"));
        assert!(!match_pattern("test", "z"));
    }

    #[test]
    fn test_match_single_character_class() {
        assert!(match_pattern("abc", "[abc]"));
        assert!(!match_pattern("xyz", "[abc]"));
    }

    #[test]
    #[should_panic(expected = "Unhandled pattern")]
    fn test_unhandled_pattern_panics() {
        match_pattern("test", "unknown");
    }
}
