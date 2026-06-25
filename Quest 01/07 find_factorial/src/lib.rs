pub fn factorial(num: u64) -> u64 {
    let mut sum = 1;
    if num == 0 && num == 1 {
        return sum;
    }
    for i in 1..=num {
        sum *= i;
    }
    return sum;
}