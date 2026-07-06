pub fn number_logic(num: u32) -> bool {
    let mut vec = Vec::new();
    let mut com = num.clone();
    let mut sum = 0;
    let mut reff = 0;
    while com > 0 {
        vec.push(com%10);
        com /= 10;
        reff += 1;
    }
    for n in vec {
        sum += n.pow(reff);
    }
    sum == num
}