use regex::Regex;

#[allow(dead_code)]
pub fn is_valid_hex128(input: &str) -> bool {
    let re = unsafe { Regex::new(r"(?i)^[0-9a-f]{32}$").unwrap_unchecked() };
    re.is_match(input)
}

pub fn extract_32byte_hex(input: &str) -> Option<String> {
    let re = unsafe { Regex::new(r"(?i)[0-9a-f]{32}").unwrap_unchecked() };

    re.find(input).map(|m| m.as_str().to_lowercase())
}
