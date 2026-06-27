use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut bigger = 0;
    for (k,v) in &h {
        if *v > bigger {
            bigger = *v;
        }
    }
    bigger
}
