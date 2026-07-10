use std::rc::Rc;
use std::cell::RefCell;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: i32,
    pub max: i32
}

impl Tracker {
    pub fn new(m: i32) -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
            value: 1,
            max: m,
        }
    }

    pub fn set_value(&mut self ,v : &Rc<i32>) {
        let p = ( Rc::strong_count(v) as f64 / self.max as f64) * 100.0;
       //println!("{}",Rc::strong_count(v));
        if p > 100.0 {
            self.messages.borrow_mut().push(format!("Error: You can't go over your quota!"))
        } else if p > 70.0 {
            self.messages.borrow_mut().push(format!("Warning: You have used up over {}% of your quota!", p.round()))
        }

    }

    pub fn peek(&mut self ,v : &Rc<i32>) {
        let p = ( Rc::strong_count(v) as f64 / self.max as f64) * 100.0;
        self.messages.borrow_mut().push(format!("Info: This value would use {}% of your quota", p.round()))
    }
}