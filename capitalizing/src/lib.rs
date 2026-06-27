pub fn capitalize_first(input: &str) -> String {
    let mut vector = Vec::new();
    if input.is_empty() {
        return String::new();
    }
    vector.push(input.chars().next().unwrap().to_string().to_uppercase());
    for r in input.chars().skip(1) {
        vector.push(r.to_string());
    }
    vector.join("")
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;

    for c in input.chars() {
        if c.is_whitespace() {
            capitalize = true;
            result.push(c);
        } else if capitalize {
            result.extend(c.to_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }

    result
}


pub fn change_case(input: &str) -> String {
    let mut vector = Vec::new();
    for r in input.chars() {
        if r.is_uppercase() {
            vector.push(r.to_string().to_lowercase());
        } else if r.is_lowercase() {
            vector.push(r.to_string().to_uppercase());
        } else if r == ' ' {
            vector.push(r.to_string());

        }
    }
    vector.join("")
}