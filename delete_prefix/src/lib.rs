pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    let len = prefix.len();
    let pre: String = s.chars().take(len).collect();
    if pre == prefix {
        return Some(&s[len..])
    }
    None
}