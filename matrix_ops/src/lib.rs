use std::ops::{Add, Sub, Mul};
use lalgebra_scalar::Scalar;
use matrix::Matrix;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
    fn from(arr: [[T; W]; H]) -> Self {
        Wrapper(Matrix(arr))
    }
}

impl<const W: usize, const H: usize, T: Scalar<Item = T> + Copy> Add for Wrapper<W, H, T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = self;

        for i in 0..H {
            for j in 0..W {
                res.0.0[i][j] = res.0.0[i][j] + other.0.0[i][j];
            }
        }

        res
    }
}

impl<const W: usize, const H: usize, T: Scalar<Item = T> + Copy> Sub for Wrapper<W, H, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = self;

        for i in 0..H {
            for j in 0..W {
                res.0.0[i][j] = res.0.0[i][j] - other.0.0[i][j];
            }
        }

        res
    }
}

impl<const S: usize, T: Scalar<Item = T> + Copy> Mul for Wrapper<S, S, T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut res = [[T::zero(); S]; S];

        for i in 0..S {
            for j in 0..S {
                for k in 0..S {
                    res[i][j] = res[i][j] 
                        + self.0.0[i][k] * other.0.0[k][j];
                }
            }
        }

        Wrapper(Matrix(res))
    }
}