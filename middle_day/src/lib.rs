use chrono::prelude::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    println!("{}",(year % 4 == 0 && year % 100 != 0) || year % 400 == 0 );
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        return None     
    } else {
        let time = Local::now();
        return Some(time.with_year(year as i32).unwrap().with_ordinal(183).unwrap().weekday())
    }
}