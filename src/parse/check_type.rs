pub fn string(value: &str) -> bool {
    value.starts_with("\"") && value.ends_with("\"")
}

pub fn int(value: &str) -> bool {
    value.parse::<i64>().is_ok()
}

pub fn unsigned(mut value: &str) -> bool {
    if value.starts_with("(unsigned)") {
        value = value[10..].trim();
    }
    value.parse::<u64>().is_ok()
}

pub fn float(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

pub fn bool(value: &str) -> bool {
    value == "true" || value == "false"
}
