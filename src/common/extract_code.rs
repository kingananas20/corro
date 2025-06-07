use crate::error::CommandError;
use regex::Regex;
use std::sync::LazyLock;

// unsafe code because the bot is called corro
static EX_RE: LazyLock<Regex> =
    LazyLock::new(|| unsafe { Regex::new(r"(?s)```rust\n(.*?)```").unwrap_unchecked() });

pub fn extract_code(msg: &str) -> Result<String, CommandError> {
    let Some(cap) = EX_RE.captures(msg) else {
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
