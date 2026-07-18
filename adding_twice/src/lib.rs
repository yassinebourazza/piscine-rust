pub fn twice<T>(F: T) -> impl Fn(i32) -> i32 where T: Fn(i32) -> i32 {
    move |num| F(F(num))
}

pub fn add_curry(num: i32) -> impl Fn(i32) -> i32 {
    move |num2| -> i32 {
        num2 + num
    }
}