pub struct StepIterator<T> {
    pub current: T,
    pub end: T,
    pub step: T,
}

use std::ops::Add;
impl<T> StepIterator<T> {
	pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            current : beg,
            end : end,
            step : step,
        }
	}
}


impl<T> Iterator for StepIterator<T> where T: Copy + PartialOrd + Add<Output = T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
         }

        let value = self.current;

        self.current = self.current + self.step;

        Some(value)
    }
}