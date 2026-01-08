const EMBED_DESCRIPTION_SIZE: usize = 4096;

pub fn split_content(content: &str, chunk_size: usize) -> Vec<&str> {
    if chunk_size == 0 {
        return Vec::new();
    }

    let len = content.len();
    let mut result = Vec::new();
    let estimated = (len + chunk_size - 1).div_ceil(chunk_size);
    result.reserve(estimated);

    let mut start = 0;
    while start < len {
        let mut end = (start + chunk_size).min(len);

        if !content.is_char_boundary(end) {
            while end > start && !content.is_char_boundary(end) {
                end -= 1;
            }

            if end == start {
                end = (start + chunk_size).min(len);
                while end < len && !content.is_char_boundary(end) {
                    end += 1;
                }
            }
        }

        result.push(&content[start..end]);
        start = end;
    }

    result
}

pub fn split_content_embed(content: &str) -> Vec<&str> {
    split_content(content, EMBED_DESCRIPTION_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let input = "";
        let chunks = split_content(input, 5);
        assert!(chunks.is_empty());
    }

    #[test]
    fn chunk_size_zero() {
        let input = "abc";
        let chunks = split_content(input, 0);
        assert!(chunks.is_empty());
    }

    #[test]
    fn ascii_exact_chunks() {
        let input = "abcdefghij"; // 10 bytes
        let chunks = split_content(input, 5);
        assert_eq!(chunks, vec!["abcde", "fghij"]);
    }

    #[test]
    fn ascii_non_exact_chunks() {
        let input = "abcdefghij"; // 10 bytes
        let chunks = split_content(input, 6);
        assert_eq!(chunks, vec!["abcdef", "ghij"]);
    }

    #[test]
    fn utf8_boundary_respected() {
        // "€" = 3 bytes in UTF-8
        let input = "a€b€c";
        // bytes: a(1) €(3) b(1) €(3) c(1) = 9 bytes
        let chunks = split_content(input, 4);

        // cannot split inside '€'
        assert_eq!(chunks, vec!["a€", "b€", "c"]);
    }

    #[test]
    fn large_utf8_char_larger_than_chunk() {
        // emoji is 4 bytes
        let input = "😀a";
        let chunks = split_content(input, 3);

        // chunk_size < utf8 char size, must still make progress
        assert_eq!(chunks, vec!["😀", "a"]);
    }

    #[test]
    fn no_data_loss_or_reordering() {
        let input = "hello😀world€!";
        let chunks = split_content(input, 5);

        let reconstructed = chunks.concat();
        assert_eq!(reconstructed, input);
    }

    #[test]
    fn borrowed_slices_only() {
        let input = String::from("abcdef");
        let chunks = split_content(&input, 2);

        // ensure slices point into the original string
        assert_eq!(chunks[0].as_ptr(), input.as_ptr());
    }

    #[test]
    fn embed_size_works() {
        let input = "abc";
        let chunks = split_content_embed(input);
        assert_eq!(chunks, vec!["abc"]);
    }
}
