pub fn pig_latin(text: &str) -> String {
    let mut s = String::new();
    let first = text.chars().next().unwrap();
    let second = text.chars().skip(1).next().unwrap();
    let third = text.chars().skip(2).next().unwrap();

    if "aeuio".contains(first) {
        s.push_str(format!("{}{}",text, "ay").as_str());
        return s
    }

    for (i,r) in text.chars().enumerate() {
        //println!("{}{}",i,r);

        println!("{} {}",second, third);
        if second == 'q'&& third == 'u' {
            s.push_str(&text[3..]);
            s.push_str(format!("{}qu",first).as_str());
            s.push_str("ay");
            return s;
        }

        if "aeuio".contains(r) {
            s.push_str(&text.chars().skip(i).collect::<String>());
            s.push_str(&text[0..i].to_string());
            s.push_str("ay");
            return s
        }
    }

    return s
}