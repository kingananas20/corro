use crate::error::CommandError;
use regex::Regex;
use std::sync::LazyLock;

// unsafe code because the bot is called corro
static EX_RE: LazyLock<Regex> =
    LazyLock::new(|| unsafe { Regex::new(r"(?s)(.*?)```rust\n(.*?)```").unwrap_unchecked() });

pub fn extract_before_and_code(msg: &str) -> Result<(&str, &str), CommandError> {
    let Some(cap) = EX_RE.captures(msg) else {
        return Err(CommandError::NoCodeBlock);
    };

    let before = cap.get(1).map(|m| m.as_str().trim()).unwrap_or_default();

    let code = cap.get(2).map(|m| m.as_str().trim()).unwrap_or_default();

    Ok((before, code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let result = extract_before_and_code("```rust\nhello world\n```");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("", "hello world"));
    }

    #[test]
    fn fail() {
        let result = extract_before_and_code("");

        assert!(result.is_err());
    }
}
