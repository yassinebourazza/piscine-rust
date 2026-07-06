pub fn expected_variable(str1: &str, str2: &str) -> Option<String> {

    if !(!str1.is_empty() && str1.chars().all(|c| (c.is_alphanumeric() || c == '_'))) {
        return None;
    }

    if str2.is_empty() {
        return None;
    }
    let compare_lower = str1.to_lowercase();
    let expected_lower = str2.to_lowercase();

    let distance = edit_distance(&str1.to_lowercase(), &str2.to_lowercase());


    let res = ((str2.len().saturating_sub(distance) as f64) / (str2.len() as f64)) * 100.0;
    let pre = res.round() as i32;

    if pre > 50 {
        Some(format!("{}%", pre))
    } else {
        None
    }
}

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
