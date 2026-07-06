pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    let mut key = key;

    if key < 0 {
        key += 26;
    }

    let key = key as u8;

    for c in input.chars() {
        if c.is_ascii_lowercase() {
            let new = (((c as u8 - b'a') + key) % 26 + b'a') as char;
            res.push(new);
        } else if c.is_ascii_uppercase() {
            let new = (((c as u8 - b'A') + key) % 26 + b'A') as char;
            res.push(new);
        } else {
            res.push(c);
        }
    }
    res
}