pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;
    let mut res = Vec::new();

    let len = n * 2 + 1;

    for i in 0..=n {
        let cha = (b'A' + i as u8) as char;
        let mut line = String::new();

        for j in 0..len {
            if j == n - i || j == n + i {
                line.push(cha);
            } else {
                line.push(' ');
            }
        }

        res.push(line);
    }

    for i in (0..n).rev() {
        res.push(res[i].clone());
    }

    res
}