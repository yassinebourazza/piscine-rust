pub fn arrange_phrase(phrase: &str) -> String {
    let mut res = vec![String::new(); phrase.split_whitespace().count()];
    for r in phrase.split_whitespace() {
        let stri = r.chars().filter(|c| !c.is_numeric()).collect::<String>();
        let num = r.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<usize>().unwrap();
        res[num-1] = stri
    }
    return res.join(" ")
}