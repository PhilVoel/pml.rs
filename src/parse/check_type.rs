pub fn string(value: &str) -> bool {
    value.starts_with("\"") && value.ends_with("\"")
}

pub fn int(value: &str) -> bool {
    value.parse::<i128>().is_ok()
}

pub fn unsigned(mut value: &str) -> bool {
    if value.starts_with("(u)") {
        value = value[10..].trim();
    }
    value.parse::<u128>().is_ok()
}

pub fn float(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

pub fn bool(value: &str) -> bool {
    value == "true" || value == "false"
}
