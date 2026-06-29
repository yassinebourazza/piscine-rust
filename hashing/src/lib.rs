use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    ( list.iter().sum::<i32>() as f64) / (list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut vec = list.to_vec();
    vec.sort();
    if vec.len() % 2 == 1 {
        return vec[vec.len()/2]
    } else {
        return (vec[vec.len()/2] + vec[(vec.len()/2)-1]) /2
    }
    
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for &num in list {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut max = 0;
    let mut mode = 0;

    for (num, count) in map {
        if count > max {
            max = count;
            mode = num;
        }
    }

    mode
}