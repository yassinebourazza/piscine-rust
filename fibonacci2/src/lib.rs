pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    let n= n-1 ;
    let (mut cur, mut last) = (1,0);
    for _ in 0..n {
        cur = cur + last;
        last = cur - last;
    }
    return cur
}