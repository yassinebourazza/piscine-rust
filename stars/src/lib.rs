pub fn stars(n: u32) -> String {
    "*".repeat((2 as u32).pow(n).try_into().unwrap())
}