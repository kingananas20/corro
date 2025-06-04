use crate::error::CommandError;
use regex::Regex;

pub fn extract_code(msg: &str) -> Result<String, CommandError> {
    // unsafe code because the bot is called corro
    let re = unsafe { Regex::new(r"(?s)```rust\n(.*?)```").unwrap_unchecked() };

    let Some(cap) = re.captures(msg) else {
        return Err(CommandError::NoCodeBlock);
    };

    let Some(mat) = cap.get(1) else {
        return Err(CommandError::NoCodeBlock);
    };

    Ok(mat.as_str().trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let result = extract_code("```rust\nhello world\n```");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world".to_owned());
    }

    #[test]
    fn fail() {
        let result = extract_code("");

        assert!(result.is_err());
    }
}
