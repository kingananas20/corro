const EMBED_DESCRIPTION_SIZE: usize = 4096;

pub fn split_content(content: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;

    while start < content.len() {
        let mut end = (start + EMBED_DESCRIPTION_SIZE).min(content.len());

        while !content.is_char_boundary(end) {
            end -= 1;
        }

        if end <= start {
            end = (start + EMBED_DESCRIPTION_SIZE).min(content.len());
        }

        println!("{start}, {end}");

        result.push(content[start..end].to_string());
        start = end;
    }

    result
}
