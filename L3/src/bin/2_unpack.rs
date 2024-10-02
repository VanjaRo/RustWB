use std::error::Error;
use std::fmt;

// User custom error
#[derive(Debug)]
struct UnpackError(String);

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid input string: {}", self.0)
    }
}

impl Error for UnpackError {}

fn unpack_string(input: &str) -> Result<String, UnpackError> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    let mut escape = false;

    while let Some(c) = chars.next() {
        if escape {
            // Add symbol as it is
            result.push(c);
            escape = false;
        } else if c == '\\' {
            // Found ecape char
            escape = true;
        } else if c.is_digit(10) {
            // Not enough chars to repeat
            if result.len() == 0 {
                return Err(UnpackError(format!("Invalid start with digit: {}", c)));
            }
            let prev_char = result.pop().unwrap();
            let mut repeat_count = c.to_digit(10).unwrap();
            while let Some(&next_char) = chars.peek() {
                if next_char.is_digit(10) {
                    // Here 10 is a base
                    repeat_count = repeat_count * 10 + next_char.to_digit(10).unwrap();
                    chars.next();
                } else {
                    break;
                }
            }
            for _ in 0..repeat_count {
                result.push(prev_char);
            }
        } else {
            // This is symbol that can be repeated
            result.push(c);
        }
    }

    if escape {
        // Finishing string with incomplete escape sequence
        return Err(UnpackError("Incomplete escape sequence".to_string()));
    }

    Ok(result)
}

// Unit-тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack() {
        assert_eq!(unpack_string("a4bc2d5e").unwrap(), "aaaabccddddde");
        assert_eq!(unpack_string("abcd").unwrap(), "abcd");
        assert_eq!(unpack_string("").unwrap(), "");
        assert_eq!(unpack_string("a10").unwrap(), "aaaaaaaaaa");
        assert!(unpack_string("45").is_err());
    }

    #[test]
    fn test_escape_sequences() {
        assert_eq!(unpack_string(r"qwe\4\5").unwrap(), "qwe45");
        assert_eq!(unpack_string(r"qwe\45").unwrap(), "qwe44444");
        assert_eq!(unpack_string(r"qwe\\5").unwrap(), r"qwe\\\\\");
    }

    #[test]
    fn test_invalid_escape_sequence() {
        assert!(unpack_string(r"qwe\").is_err());
        assert!(unpack_string(r"\").is_err());
    }
}

fn main() {
    match unpack_string("a4bc2d5e") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
