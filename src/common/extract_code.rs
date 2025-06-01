use regex::Regex;

pub fn extract_code(msg: &str) -> String {
    let re = Regex::new(r"(?s)```(?:rust)?\n(.*?)```").unwrap();

    if let Some(cap) = re.captures(msg) {
        cap.get(1).unwrap().as_str().to_string()
    } else {
        String::new()
    }
}
