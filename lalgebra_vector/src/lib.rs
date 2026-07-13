use std::ops::Add;
use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);


impl<T: Add<Output = T> + Scalar> Add for Vector<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut res = Vector(vec![]);
        for i in 0..self.0.len() {
            res.0.push(self.0[i] + other.0[i]);
        }
        Some(res)
    }
}

impl<T: Scalar + std::ops::AddAssign> Vector<T> {
    pub fn new() -> Self {
        Vector(vec![])
    }

    pub fn dot(self, other: Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut res = vec![];
        for i in 0..self.0.len() {
            let r = self.0[i] * other.0[i];
            res.push(r);
        }
        let mut sum = res[0];
        for i in 1..res.len() {
            sum += res[i];
        }
        Some(sum)
    }
}