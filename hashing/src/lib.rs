
pub fn mean(list: &[i32]) -> f64 {
    ( list.iter().sum::<i32>() as f64) / (list.len() as f64)
}

//pub fn median(list: &[i32]) -> i32 {
//    if list.len() % 2 == 0 {
//        return list.chars().nth(list.len()/2)
//    }
//}

//pub fn mode(list: &[i32]) -> i32 {
//    list.mode()
//}