pub fn escape_triple_backticks(s: &str) -> String {
    s.replace("```", "```\u{200B}")
}

