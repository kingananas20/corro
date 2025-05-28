const EMBED_DESCRIPTION_SIZE: usize = 4096;

pub fn split_content(content: String) -> Vec<String> {
    let n = content.len() % EMBED_DESCRIPTION_SIZE;
    let mut result = Vec::new();

    for i in 0..n {
        let s = content[i * EMBED_DESCRIPTION_SIZE..(i + 1) * EMBED_DESCRIPTION_SIZE].to_owned();
        result.push(s);
    }

    result
}
