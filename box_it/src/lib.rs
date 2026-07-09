pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut vec = Vec::new();
    for st in s.split_whitespace() {
        if st.contains("k") {
            let num: f32 = st.trim_end_matches('k').parse().unwrap();
            vec.push(Box::new((num * 1000.0) as u32))
        } else {
            let num = st.parse().unwrap();
            vec.push(Box::new(num))
        }
    }
    vec
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut vec = Vec::new();

    for st in a {
        vec.push(*st)
    }
    vec
}
