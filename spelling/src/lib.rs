pub fn spell(n: u64) -> String {
    let mut num = n.clone();
    let mut res = Vec::new();
    if num == 0 {
        return "zero".to_string()
    }
    while num > 0 {
        //println!("{:?}  {}",res,num);
        if num < 10 {
            res.push(nums(num));
            num /= 10;
            break
        } else if num >= 10 && num < 100 {
            if num == 10 {
                res.push("ten".to_string());
                break
            } else if num > 10 && num < 20 {
                res.push(teen(num));
                break
            } else {
                if num % 10 == 0 {
                    res.push(tens((num/10) * 10));
                } else {
                    res.push(format!("{}-{}",tens((num/10) * 10), nums(num%10)));
                }
                break
            }
        } else if num >= 100 && num < 1000 {

            res.push(format!("{} hundred",nums(num/100)));
            num %= 100;
        } else if num >= 1000 && num < 1000000 {
            res.push(format!("{} thousand", spell(num/1000)));
            num %= 1000
        } else if num >= 1000000 {
            res.push("one million".to_string());
            num %= 1000000
        }
    }

    res.join(" ")
}

pub fn teen(num: u64) -> String {
    match num {
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => todo!()
    }
}
pub fn nums(num: u64) -> String {
    match num {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => todo!()
    }
    .to_string()
}

pub fn tens(num: u64) -> String {
    match num {
        10 => "ten",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => todo!()
    }
    .to_string()
}