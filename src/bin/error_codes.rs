use corro::config::logging::{LogLevel, LoggingConfig, OutputFormat};
use corro::setup_logging;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use tracing::{debug, info};

/// Main transform function
pub fn transform_text_general(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut para_buf = String::new();
    let mut inside_code = false;

    // Helper: flush paragraph buffer into output (collapse whitespace)
    let flush_para = |out: &mut String, buf: &mut String| {
        if !buf.is_empty() {
            let collapsed = buf.split_whitespace().collect::<Vec<_>>().join(" ");
            out.push_str(&collapsed);
            out.push('\n');
            buf.clear();
        }
    };

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        // code-fence handling (toggle)
        if line.starts_with("```") {
            flush_para(&mut out, &mut para_buf);
            if !inside_code {
                out.push_str("```rust\n");
                inside_code = true;
            } else {
                out.push_str("```\n");
                inside_code = false;
            }
            continue;
        }

        if inside_code {
            out.push_str(line);
            out.push('\n');
            continue;
        }

        let trimmed = line.trim_start();

        // blank line -> paragraph boundary
        if trimmed.is_empty() {
            flush_para(&mut out, &mut para_buf);
            out.push('\n');
            continue;
        }

        let is_indented_code = line.starts_with("    ") || line.starts_with('\t');
        let is_heading = trimmed.starts_with('#');
        let is_blockquote = trimmed.starts_with('>');
        let is_list = is_list_item(trimmed);

        if is_list {
            // flush current paragraph then collapse this list item with following continuation lines
            flush_para(&mut out, &mut para_buf);

            let (marker, rest) = extract_list_marker(trimmed);
            let mut item_buf = rest.trim_start().to_string();

            while let Some(peek) = lines.peek() {
                let pl = *peek;
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
                // consume and append continuation
                let cont = lines.next().unwrap();
                if !item_buf.is_empty() {
                    item_buf.push(' ');
                }
                item_buf.push_str(cont.trim_start());
            }

            if marker.is_empty() {
                out.push_str(&format!("{item_buf}\n"));
            } else {
                out.push_str(&format!("{marker}{item_buf}\n"));
            }
            continue;
        }

        if is_indented_code || is_heading || is_blockquote {
            flush_para(&mut out, &mut para_buf);
            out.push_str(line);
            out.push('\n');
            continue;
        }

        // normal paragraph accumulation
        if !para_buf.is_empty() {
            para_buf.push(' ');
        }
        para_buf.push_str(trimmed);
    }

    // final flush
    flush_para(&mut out, &mut para_buf);

    // perform link conversion
    let mut out = convert_links_final(&out);

    // Trim trailing blank lines to at most one newline
    while out.ends_with("\n\n") {
        out.pop();
    }

    out
}

fn is_list_item(trimmed: &str) -> bool {
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
        return true;
    }
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
    // unordered marker
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
        return (trimmed[..2].to_string(), &trimmed[2..]);
    }

    // numbered marker: mimic original logic
    let mut idx = 0usize;
    for (i, ch) in trimmed.char_indices() {
        if ch.is_ascii_digit() {
            idx = i + ch.len_utf8();
            continue;
        }
        if ch == '.' {
            // expect a space after dot (original used get(after..after+1) == Some(" "))
            let after = idx + 1;
            if trimmed.get(after..after + 1) == Some(" ") {
                // include digits, dot and trailing space in marker
                let marker = trimmed[..after + 1].to_string();
                let rest = &trimmed[after + 1..];
                return (marker, rest);
            }
        }
        break;
    }

    // fallback
    (String::new(), trimmed)
}

/// Convert reference-style links
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

    // `[text][]` -> `[text](url)` where label == text
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
                // preserve original check: examine the previous byte as char (only checks for '(')
                if out.as_bytes()[start - 1] as char == '(' {
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
    setup_logging(&LoggingConfig {
        output_format: OutputFormat::Default,
        log_level: LogLevel::Trace,
    });
    let re = Regex::new(r"^assets/error_codes/E\d{4}\.md$").unwrap();

    for entry in fs::read_dir("assets/error_codes")? {
        let file = entry?;
        let path = file.path();

        if let Some(path_str) = path.to_str() {
            if !re.is_match(path_str) && path.is_file() {
                continue;
            }
        }

        debug!("Starting conversion for {path:?}");
        let content = fs::read_to_string(&path)?;
        let transformed = transform_text_general(&content);
        fs::write(&path, transformed)?;
        info!("Conversion for {path:?} done");
    }

    Ok(())
}
