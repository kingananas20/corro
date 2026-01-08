use std::borrow::Cow;

pub fn limit_string<'a>(input: &'a str, max_lines: usize, max_bytes: usize) -> Cow<'a, str> {
    if max_lines == 0 || max_bytes == 0 {
        return Cow::Borrowed("");
    }

    let bytes = input.as_bytes();
    let mut scan = 0usize;
    let mut lines_taken = 0usize;

    for line in input.lines() {
        scan += line.len();
        lines_taken += 1;

        if lines_taken == max_lines {
            break;
        }

        while scan < bytes.len() && (bytes[scan] == b'\n' || bytes[scan] == b'\r') {
            scan += 1;
        }
    }

    if scan == input.len() && input.len() <= max_bytes {
        return Cow::Borrowed(input);
    }

    let mut end = scan.min(max_bytes);
    while end > 0 && !input.is_char_boundary(end) {
        end -= 1;
    }

    if end == input.len() {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(input[..end].to_string())
    }
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
