use regex::Regex;
use std::sync::LazyLock;

static HEX_32_RE: LazyLock<Regex> =
    LazyLock::new(|| unsafe { Regex::new(r"(?i)[0-9a-f]{32}").unwrap_unchecked() });

pub fn extract_32byte_hex(input: &str) -> Option<String> {
    HEX_32_RE.find(input).map(|m| m.as_str().to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let testcases = [
            "730ccb458bc9ea43ac0d14eceb7eb40b",
            "#730ccb458bc9ea43ac0d14eceb7eb40b",
            "https://gist.github.com/kingananas20/730ccb458bc9ea43ac0d14eceb7eb40b",
            "<script src='https://gist.github.com/kingananas20/730ccb458bc9ea43ac0d14eceb7eb40b.js'></script>",
        ];

        for test in testcases {
            let id = match extract_32byte_hex(test) {
                Some(id) => id,
                None => continue,
            };

            assert_eq!(id, "730ccb458bc9ea43ac0d14eceb7eb40b".to_owned());
        }
    }
}
