pub fn num_to_ordinal(x: u32) -> String {
    match x%10 {
        1 => return format!("{}st",x),
        2 => return format!("{}nd",x),
        3 => return format!("{}rd",x),
        _ => return format!("{}th",x)
    }
}