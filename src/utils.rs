fn is_valid_name(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some('_') => s.len() > 1,
        Some(c) => c.is_ascii_alphabetic(),
        None => false,
    }
}
