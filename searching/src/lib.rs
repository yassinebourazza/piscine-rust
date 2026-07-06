pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut index: Option<usize> = None;

    for (i,r) in array.iter().enumerate() {
        if key == *r {
            index = Some(i)
        }
    }
    index
}