use dbug::blocking::Connection;
use std::time::Duration;

pub fn get_dbus_property(interface: &str, property: &str) -> Result<String, String> {
	let conn = Connection::new_system().map_err(|e| e.to_string())?;
	let proxy = conn.with_proxy("org.freedesktop.hostname1", "/org/freedesktop/hostname1");

	let (value,): (String,) = proxy.get(interface, property).map_err(|e| e.to_string())?;
	Ok(value)
}

pub fn split_output(part: &str) -> Result<Vec<&str>, &str> {
    let trimmed_output: &str = part.trim();

    if trimmed_output.is_empty() {
        return Err("Warning: Cannot split empty String!");
    }
    if !trimmed_output.contains(":") {
        return Err("Error: Could not split output!");
    }
    let split: Vec<&str> = trimmed_output.split(":").collect();
    if split.len() != 2 {
        return Err("Error: Invalid number of parts in line!");
    }
    Ok(split)
}

pub fn find_table(table_header: &str, part: &str) -> bool {
    let mut table_found: bool = false;

    if part.contains(table_header) {
        table_found = true;
    }
    table_found
}
