#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u32,
}

impl<'a> Person<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name : name,
            age : 0
        }
    }
}