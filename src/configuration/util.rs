/// Helper function: Replace a single `key = value` pair in a specific TOML section.
///
/// # Parameters
/// * `text: String` - The full TOML text.
/// * `section: &str` - The section name, e.g., `"common"`.
/// * `key: &str` - The key name to replace.
/// * `new_value: &str` - The new value to assign.
///
/// # Returns
/// Modified TOML as `String`.
pub fn replace_key(mut text: String, section: &str, key: &str, new_value: &str) -> String {
    let mut in_section = false;
    let mut new_lines = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Detect section headers
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_section = trimmed == format!("[{}]", section);
            new_lines.push(line.to_string());
            continue;
        }

        // Replace line if we’re in the correct section and key matches
        if in_section && trimmed.starts_with(&format!("{key} =")) {
            // Write numeric values without quotes
            if new_value.parse::<i64>().is_ok() || new_value.parse::<f64>().is_ok() {
                new_lines.push(format!("{key} = {}", new_value));
            } else {
                new_lines.push(format!("{key} = \"{}\"", new_value));
            }
        } else {
            new_lines.push(line.to_string());
        }
    }

    text.clear();
    text.push_str(&new_lines.join("\n"));
    text
}

/// Removes the starting comment on a specific key
pub fn uncomment_key(mut text: String, section: &str, key: &str) -> String {
    let mut in_section = false;
    let mut new_lines = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Detect section headers
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_section = trimmed == format!("[{}]", section);
            new_lines.push(line.to_string());
            continue;
        }

        // Replace line if we’re in the correct section and key matches
        if in_section && trimmed.starts_with(&format!("# {key} =")) {
            let mut string = line.to_string();
            string.remove(0);
            string.remove(0);
            new_lines.push(string);
        } else {
            new_lines.push(line.to_string());
        }
    }

    text.clear();
    text.push_str(&new_lines.join("\n"));
    text
}

/// Removes the starting comment on a specific section
pub fn uncomment_section(mut text: String, section: &str) -> String {
    let mut new_lines = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Detect section headers
        if trimmed.starts_with(&format!("# [{section}")) && trimmed.ends_with(']') {
            let mut string = line.to_string();
            string.remove(0);
            string.remove(0);
            new_lines.push(string);
        } else {
            new_lines.push(line.to_string());
        }
    }

    text.clear();
    text.push_str(&new_lines.join("\n"));
    text
}

/// Adds a starting comment on a specific key
pub fn comment_out_key(mut text: String, section: &str, key: &str) -> String {
    let mut in_section = false;
    let mut new_lines = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Detect section headers
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_section = trimmed == format!("[{}]", section);
            new_lines.push(line.to_string());
            continue;
        }

        // Replace line if we’re in the correct section and key matches
        if in_section && trimmed.starts_with(&format!("{key} =")) {
            new_lines.push(format!("# {}", trimmed));
        } else {
            new_lines.push(line.to_string());
        }
    }

    text.clear();
    text.push_str(&new_lines.join("\n"));
    text
}

/// Adds a starting comment on a specific section
pub fn comment_out_section(mut text: String, section: &str) -> String {
    let mut new_lines = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Detect section headers
        if trimmed.starts_with(&format!("[{section}")) && trimmed.ends_with(']') {
            new_lines.push(format!("# {}", line));
        } else {
            new_lines.push(line.to_string());
        }
    }

    text.clear();
    text.push_str(&new_lines.join("\n"));
    text
}
