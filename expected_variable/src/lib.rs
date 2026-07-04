pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let is_valid_casing =
        !compare.is_empty() && compare.chars().all(|c| (c.isalphanumeric() || c == ''));

    if !is_valid_casing {
        return None;
    }

    if expected.is_empty() {
        return None;
    }
    let compare_lower = compare.to_lowercase();
    let expected_lower = expected.to_lowercase();

    let distance = edit_distance(&compare_lower, &expected_lower);

    let expected_len = expected_lower.len();

    let alikeness =
        ((expected_len.saturating_sub(distance) as f64) / (expected_len as f64)) * 100.0;
    let percentage = alikeness.round() as i32;

    if percentage > 50 {
        Some(format!("{}%", percentage))
    } else {
        None
    }
}