use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut col = Vec::new();

        for row in &self.0 {
            col.push(row[n]);
        }

        col
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T>+ std::ops::AddAssign> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Self::Output {
        if self.number_of_rows() != other.number_of_rows()
        || self.number_of_cols() != other.number_of_cols() {
            return None;
        }

        let mut res = vec![
            vec![T::default(); other.number_of_cols()];
            self.number_of_rows()
        ];

        for i in 0..self.number_of_rows() {

            for j in 0..other.number_of_cols() {
                for k in 0..self.number_of_cols() {

                    res[i][j] += self.0[i][k] * other.0[k][j];
                }

            }
        }

        Some(Matrix(res))
    }
}