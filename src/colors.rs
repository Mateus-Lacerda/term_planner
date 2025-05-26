pub fn colored(text: &str, color: &str) -> String {
    match color {
        "yellow" => format!("\x1b[33m{text}\x1b[0m"),
        "green" => format!("\x1b[32m{text}\x1b[0m"),
        "blue" => format!("\x1b[34m{text}\x1b[0m"),
        "red" => format!("\x1b[31m{text}\x1b[0m"),
        _ => text.to_string(),
    }
}
