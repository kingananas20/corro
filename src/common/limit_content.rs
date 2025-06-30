/// FIXME: Doesn't work correctly. Maybe also change to embed output for code output
pub fn limit_string(input: &str, max_lines: usize, max_length: usize) -> String {
    let limited_lines = input
        .lines()
        .take(max_lines)
        .collect::<Vec<&str>>()
        .join("\n");

    let bytes = limited_lines.as_bytes();
    if bytes.len() <= max_length {
        return limited_lines;
    }

    let mut end = max_length;
    while !limited_lines.is_char_boundary(end) {
        end -= 1;
    }

    limited_lines[..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let strings = [
            String::new(),
            String::from("some text"),
            "j".repeat(2000),
            "j".repeat(2100),
        ];

        let results = [
            String::new(),
            String::from("some text"),
            "j".repeat(2000),
            "j".repeat(2000),
        ];

        for (i, string) in strings.iter().enumerate() {
            println!("index: {i}");
            let string = limit_string(string, 100, 2000);
            assert_eq!(string, results[i]);
        }
    }
}
