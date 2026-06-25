pub fn fibonacci(n: u32) -> u32 {
    let (mut cur, mut last) = (1,0);
    for _ in 0..n-1 {
        cur = cur + last;
        last = cur - last;
    }
    return cur
}