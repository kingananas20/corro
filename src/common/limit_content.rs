pub(crate) fn limit_string(input: &str) -> String {
    let limited_lines = input.lines().take(100).collect::<Vec<&str>>().join("\n");

    let bytes = limited_lines.as_bytes();
    if bytes.len() <= 1994 {
        return limited_lines;
    }

    let mut end = 1994;
    while !limited_lines.is_char_boundary(end) {
        end -= 1;
    }

    limited_lines[..end].to_string()
}
