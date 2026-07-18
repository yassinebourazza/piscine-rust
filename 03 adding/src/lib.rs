pub fn add_curry(num: i32) -> impl Fn(i32) -> i32 {
    move |num2| -> i32 {
        num2 + num
    }
}