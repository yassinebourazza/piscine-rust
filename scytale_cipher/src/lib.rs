pub fn scytale_cipher(message: &str, i: usize) -> String {
    
    let mut len = message.len()/i;
    if message.len()%i != 0 {
        len += 1;
    }
    println!("{} -- {}",len,i);
    let mut arr = vec![vec![" ".to_string();len];i];
    for (j,r) in message.chars().enumerate() {
        arr[j%i][j/i] = r.to_string();
    }
    println!("{:?}",arr);
    let mut vec = Vec::new();
    for r in arr {
        vec.push(r.join(""));
    }
    vec.join("").trim().to_string()
    
}