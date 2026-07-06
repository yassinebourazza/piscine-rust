pub fn num_to_ordinal(x: u32) -> String {
      let ref = match x % 100 {
        11 | 12 | 13 => "th",
        _ => match x % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    format!("{}{}", x, ref)
}