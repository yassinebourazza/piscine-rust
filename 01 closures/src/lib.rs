pub fn first_fifty_even_square() -> Vec<i32> {
    let vec = Vec::new();
    let close = |mut vec: Vec<i32>| -> Vec<i32> {
        for i in 2..=100 {
            if i%2 == 0 {
                vec.push(i*i);
            }
        }
        vec
    };
    close(vec)
}