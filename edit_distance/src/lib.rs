pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut index = 0;
    for (i,r) in source.chars().enumerate() {
        if r == target.chars().nth(i+index).unwrap_or(' ') {
            index += 1
        }
    }
    index
}