pub fn first_subword(mut s: String) -> String {
    let st = &mut s;
    for (i,r) in st.char_indices().skip(1) {
        if r.is_uppercase() || r == '_' {
            return st[0..i].to_string()
        } 
    }
    return s
}
