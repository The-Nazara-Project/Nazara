pub fn split_output<'a>(part: &'a str) -> Result<Vec<&'a str>, &'a str> {
    let trimmed_output: &str = part.trim();

    if trimmed_output.len() == 0 {
        return Err("Warning: Cannot split empty String!");
    }
    if !trimmed_output.contains(":") {
        return Err("Error: Could not split output!");
    }
    let split: Vec<&str> = trimmed_output.split(":").collect();
    if split.len() != 2 {
        return Err("Error: Invalid number of parts in line!");
    }
    return Ok(split);
}

pub fn find_table(table_header: &str, part: &str) -> bool {
    let mut table_found: bool = false;

    if part.contains(table_header) {
        table_found = true;
    }
    return table_found;
}
