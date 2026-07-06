use std::collections::HashMap;
pub fn is_pangram(s: &str) -> bool {
    let mut map = HashMap::new();
    for r in s.chars() {
        if r.is_alphabetic() {
            *map.entry(r).or_insert(0) += 1;
        }
    }
    println!("{}",map.len());
    if map.len() == 26 {
        return true
    }
    false
}