// Converts error code markdown files to markdown supported by discord

fn transform_text_general(input: &str) -> String {
    let mut output = String::new();
    let mut in_code_block = false;
    let mut blank_line_pending = false;

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            if !in_code_block {
                // Starting a code block
                in_code_block = true;

                // Detect if compile_fail with variants, replace by ```rust
                if trimmed.starts_with("```compile_fail") {
                    output.push_str("```rust\n");
                } else {
                    // normal fence line, preserve as is
                    output.push_str(line);
                    output.push('\n');
                }
            } else {
                // Ending a code block
                in_code_block = false;
                output.push_str(line);
                output.push('\n');
            }
            blank_line_pending = false;
        } else if in_code_block {
            // Inside code block: preserve everything as is
            output.push_str(line);
            output.push('\n');
            blank_line_pending = false;
        } else {
            // Outside code block

            if trimmed.is_empty() {
                // Blank line outside code block
                // Only output if not already output a blank line
                if !blank_line_pending {
                    output.push('\n');
                    blank_line_pending = true;
                }
            } else if let Some(_rest) = trimmed.strip_prefix('[') {
                // Possibly a markdown reference link line, check for [name]: url pattern
                if let Some(colon_pos) = trimmed.find(':') {
                    // Check if format is [name]: url
                    let (name_part, url_part) = trimmed.split_at(colon_pos);
                    let url_part = url_part.trim_start_matches(':').trim();

                    // Verify name part looks like [something]
                    if name_part.starts_with('[')
                        && name_part.ends_with(']')
                        && url_part.starts_with("http")
                    {
                        // Convert to [name]: <url>
                        output.push_str(name_part);
                        output.push_str(": <");
                        output.push_str(url_part);
                        output.push_str(">\n");
                        blank_line_pending = false;
                        continue;
                    }
                }

                // Not a recognized link line, just output as normal
                output.push_str(line.trim_end());
                output.push('\n');
                blank_line_pending = false;
            } else {
                // Normal text line outside code block
                output.push_str(line.trim_end());
                output.push('\n');
                blank_line_pending = false;
            }
        }
    }

    // Trim trailing blank lines
    while output.ends_with("\n\n") {
        output.pop();
    }

    output
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::fs;

    let file = "E0801";

    let content = fs::read_to_string(format!("error_codes/{file}.md"))?;

    let transformed = transform_text_general(&content);

    fs::write(format!("error_codes/{file}_transformed.md"), transformed)?;

    Ok(())
}
