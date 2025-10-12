// Converts error code markdown files to markdown supported by discord

fn transform_text_general(input: &str) -> String {
    let mut new_text = String::new();
    let mut inside_code_block = false;
    let mut para_buf = String::new();

    let flush_para = |out: &mut String, buf: &mut String| {
        if !buf.is_empty() {
            let collapsed = buf.split_whitespace().collect::<Vec<_>>().join(" ");
            out.push_str(&collapsed);
            out.push('\n');
            buf.clear();
        }
    };

    fn is_list_item(trimmed: &str) -> bool {
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
            return true;
        }
        // numbered list like "1. " or "10. "
        let mut chars = trimmed.chars();
        let mut seen_digit = false;
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                seen_digit = true;
                continue;
            }
            if c == '.' && seen_digit {
                return chars.next().is_some_and(|n| n == ' ');
            }
            break;
        }
        false
    }

    fn extract_list_marker(trimmed: &str) -> (String, &str) {
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
            let marker = (trimmed[..2]).to_string();
            let rest = &trimmed[2..];
            return (marker, rest);
        }
        // numbered: consume digits + ". "
        let mut idx = 0usize;
        for (i, ch) in trimmed.char_indices() {
            if ch.is_ascii_digit() {
                idx = i + ch.len_utf8();
                continue;
            }
            if ch == '.' {
                // expect a space after dot
                let after = idx + 1;
                if trimmed.get(after..after + 1) == Some(" ") {
                    let marker = trimmed[..after + 1].to_string(); // includes dot and space
                    let rest = &trimmed[after + 1..];
                    return (marker, rest);
                }
            }
            break;
        }
        // fallback: treat whole line as rest with empty marker
        (String::new(), trimmed)
    }

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        // handle fenced code blocks
        if line.starts_with("```") {
            flush_para(&mut new_text, &mut para_buf);
            if !inside_code_block {
                new_text.push_str("```rust\n");
                inside_code_block = true;
            } else {
                new_text.push_str("```\n");
                inside_code_block = false;
            }
            continue;
        }

        if inside_code_block {
            new_text.push_str(line);
            new_text.push('\n');
            continue;
        }

        let trimmed = line.trim_start();
        // blank line => paragraph boundary
        if trimmed.is_empty() {
            flush_para(&mut new_text, &mut para_buf);
            new_text.push('\n');
            continue;
        }

        let is_indented_code = line.starts_with("    ") || line.starts_with('\t');
        let is_heading = trimmed.starts_with('#');
        let is_blockquote = trimmed.starts_with('>');
        let is_list = is_list_item(trimmed);

        if is_list {
            // collapse this list item with any following continuation lines
            flush_para(&mut new_text, &mut para_buf);

            let (marker, rest) = extract_list_marker(trimmed);
            let mut item_buf = rest.trim_start().to_string();

            while let Some(peek) = lines.peek() {
                let pl = peek;
                let ptrim = pl.trim_start();
                if ptrim.is_empty() {
                    break;
                }
                if is_list_item(ptrim) {
                    break;
                }
                if pl.starts_with("    ")
                    || pl.starts_with('\t')
                    || ptrim.starts_with('#')
                    || ptrim.starts_with('>')
                {
                    break;
                }
                // consume continuation line and append
                let cont = lines.next().unwrap();
                if !item_buf.is_empty() {
                    item_buf.push(' ');
                }
                item_buf.push_str(cont.trim_start());
            }

            // emit collapsed list line (preserve marker)
            if marker.is_empty() {
                new_text.push_str(&format!("{item_buf}\n"));
            } else {
                new_text.push_str(&format!("{marker}{item_buf}\n"));
            }
            continue;
        }

        if is_indented_code || is_heading || is_blockquote {
            flush_para(&mut new_text, &mut para_buf);
            new_text.push_str(line);
            new_text.push('\n');
            continue;
        }

        // normal paragraph line: accumulate
        if !para_buf.is_empty() {
            para_buf.push(' ');
        }
        para_buf.push_str(trimmed);
    }

    // final flush
    flush_para(&mut new_text, &mut para_buf);

    // perform link conversion
    new_text = convert_links_final(&new_text);

    // Trim trailing blank lines (collapse to at most one trailing newline)
    while new_text.ends_with("\n\n") {
        new_text.pop();
    }

    new_text
}

use regex::Regex;
use std::collections::HashMap;

pub fn convert_links_final(input: &str) -> String {
    // collect reference definitions `[label]: url`
    let def_re = Regex::new(r"(?m)^\s*\[([^\]]+)\]:\s*(\S+)\s*$").unwrap();
    let mut defs: HashMap<String, String> = HashMap::new();
    for cap in def_re.captures_iter(input) {
        defs.insert(cap[1].to_string(), cap[2].to_string());
    }

    // remove definition lines
    let mut s = def_re.replace_all(input, "").to_string();

    // `[text][label]` -> `[text](url)` when label exists
    let re_text_label = Regex::new(r"\[([^\]]+)\]\[([^\]]+)\]").unwrap();
    s = re_text_label
        .replace_all(&s, |caps: &regex::Captures| {
            let text = &caps[1];
            let label = &caps[2];
            if let Some(url) = defs.get(label) {
                format!("[{text}]({url})")
            } else {
                caps[0].to_string()
            }
        })
        .to_string();

    // `[text][]` -> `[text](url)` where label == text (common shorthand)
    let re_text_empty = Regex::new(r"\[([^\]]+)\]\[\]").unwrap();
    s = re_text_empty
        .replace_all(&s, |caps: &regex::Captures| {
            let text = &caps[1];
            if let Some(url) = defs.get(text) {
                format!("[{text}]({url})")
            } else {
                caps[0].to_string()
            }
        })
        .to_string();

    // standalone `[label]` -> `[label](url)` if a definition exists and it's not already inline/ref
    let mut out = String::with_capacity(s.len());
    let mut pos = 0usize;
    while let Some(open_rel) = s[pos..].find('[') {
        let open = pos + open_rel;
        out.push_str(&s[pos..open]);
        if let Some(rel_end) = s[open + 1..].find(']') {
            let end = open + 1 + rel_end;
            let next_char = s[end + 1..].chars().next();
            let is_inline_or_ref = matches!(next_char, Some('(') | Some('['));
            if !is_inline_or_ref {
                let label = &s[open + 1..end];
                if let Some(url) = defs.get(label) {
                    out.push_str(&format!("[{label}]({url})"));
                    pos = end + 1;
                    continue;
                }
            }
            out.push_str(&s[open..end + 1]);
            pos = end + 1;
        } else {
            out.push_str(&s[open..]);
            pos = s.len();
            break;
        }
    }
    if pos < s.len() {
        out.push_str(&s[pos..]);
    }

    // convert bare URLs into `[](url)` unless already inside parentheses
    let url_re = Regex::new(r"https?://\S+").unwrap();
    let out2 = url_re
        .replace_all(&out, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            let start = m.start();
            if start > 0 {
                let prev = out.as_bytes()[start - 1] as char;
                if prev == '(' {
                    return m.as_str().to_string();
                }
            }
            format!("[]({})", m.as_str())
        })
        .to_string();

    // final safety: collapse any `[text][](url)` -> `[text](url)` if it slipped through
    let collapse_re = Regex::new(r"\[([^\]]+)\]\[\]\(([^)]+)\)").unwrap();
    collapse_re.replace_all(&out2, "[$1]($2)").to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::fs;
    let re = Regex::new(r"^assets/error_codes/E\d{4}\.md$").unwrap();

    for entry in fs::read_dir("assets/error_codes")? {
        let file = entry?;
        let path = file.path();

        if let Some(path_str) = path.to_str() {
            if !re.is_match(path_str) && path.is_file() {
                continue;
            }
        }

        let content = fs::read_to_string(&path)?;
        let transformed = transform_text_general(&content);
        fs::write(&path, transformed)?;
    }

    Ok(())
}
