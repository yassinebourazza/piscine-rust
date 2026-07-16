use lalgebra_scalar::Scalar;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T: Scalar<Item = T> + Copy> Matrix<W, H, T> {
    pub fn zero() -> Self {
        Self([[T::zero(); W]; H])
    }
}

impl<const S: usize, T: Scalar<Item = T> + Copy> Matrix<S, S, T> {
    pub fn identity() -> Self {
        let mut count = 0;
        let mut matrix = [[T::zero(); S]; S];
        while count < S {
            matrix[count][count] = T::one();
            count += 1;
        }
        Self(matrix)
    }
}