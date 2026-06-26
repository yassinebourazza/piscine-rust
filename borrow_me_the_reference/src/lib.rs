pub fn delete_and_backspace(s: &mut String) {
    let mut vec = Vec::new();
    let mut index = 0;
    for (_,r) in s.char_indices() {
        if r == '+' {
            index+=1
        } else if r == '-' {
            vec.pop();
        } else if index == 0 {
            vec.push(r);
        } else {
            index-=1
        }
    }
    *s = vec.iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for i in v {
        if i.contains("+") {
            let parts : Vec<&str> = i.split('+').collect();
            let n1 : i32 = parts[0].parse().unwrap();
            let n2 : i32 = parts[1].parse().unwrap();
            *i = (n1 + n2).to_string();
        }else if i.contains('-') {
            let parts : Vec<&str> = i.split('-').collect();
            let n1 : i32 = parts[0].parse().unwrap();
            let n2 : i32 = parts[1].parse().unwrap();
            *i = (n1 - n2).to_string();
        }
    }
}