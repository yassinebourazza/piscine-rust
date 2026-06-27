use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let map1 = create_map(s1);
    let map2 = create_map(s2);
    for (k,v) in &map1 {
        if map2.get(k) != Some(v) {
            return false
        }
    }
    return true
}

pub fn create_map(s1: &str) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    for w in s1.chars() {
        *map.entry(w).or_insert(0) += 1;
    }
    map
}