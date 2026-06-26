pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c,(c as f64).exp() ,(c.abs() as f64 ).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut vec: Vec<String> = a.split(" ")
    .map(|st| {
        let n: f64 = st.parse().unwrap();
        n.exp().to_string()
    }
    ).collect();
  
    (a , vec.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut vec = Vec::new();
    for val in &b {
        vec.push((val.abs() as f64).ln());
    }
    (b, vec)

}
