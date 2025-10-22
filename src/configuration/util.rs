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
