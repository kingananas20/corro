use once_cell::sync::Lazy;
use regex::Regex;

static HEX_32_RE: Lazy<Regex> =
    Lazy::new(|| unsafe { Regex::new(r"(?i)[0-9a-f]{32}").unwrap_unchecked() });

pub fn extract_32byte_hex(input: &str) -> Option<String> {
    HEX_32_RE.find(input).map(|m| m.as_str().to_lowercase())
}
