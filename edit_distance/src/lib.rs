pub fn edit_distance(source: &str, target: &str) -> usize {
    let sc: Vec<char> = source.chars().collect();
    let tc: Vec<char> = target.chars().collect();
    let n = tc.len();
    let mut dp: Vec<usize> = (0..=n).collect();
    for i in 1..=sc.len() {
        let mut pre = dp[0];
        dp[0] = i;
        for j in 1..=n {
            let temp = dp[j];
            if sc[i - 1] == tc[j - 1] {
                dp[j] = pre;
            } else {
                dp[j] = 1 + std::cmp::min(pre, std::cmp::min(dp[j], dp[j - 1]));
            }
            pre = temp;
        }
    }
    dp[n]
}
