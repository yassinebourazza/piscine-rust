pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut vec = Vec::new();
    if arr.len < 2 {
        return vec
    }
    for (i,_n1) in arr.iter().enumerate() {
        let mut count = 1;
        for (j, n2) in arr.iter().enumerate() {
            if i != j {
                count*= n2
            }
        }
        vec.push(count)
    }
    vec
}