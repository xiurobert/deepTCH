pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    /// Creates a new matrix with the given dimensions.
    /// Initializes all elements to 0.
    ///
    /// # Arguments
    ///
    /// * `rows`: The number of rows in the matrix.
    /// * `cols`: The number of columns in the matrix.
    ///
    /// returns: Matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use dtypes::Matrix;
    ///
    /// let m = Matrix::zeros(2, 3);
    ///
    /// ```
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        let mut data = Vec::new();
        for _i in 0..rows {
            let mut row = Vec::new();
            for _j in 0..cols {
                row.push(0.0);
            }
            data.push(row);
        }
        Matrix {
            rows,
            cols,
            data
        }
    }

    /// Constructs a new matrix from a vector of data.
    pub fn new(data: Vec<Vec<f64>>) -> Matrix {
        let rows = data.len();
        let cols = data[0].len();
        // TODO: Check that the matrix is actually a matrix.
        let mut m = Matrix::zeros(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                m.set(i, j, data[i][j]);
            }
        }
        m
    }

    /// Retrieves the value at the given row and column.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    /// Sets the value at the given row and column.
    /// Note that this operation occurs inplace.
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    /// Computes the matrix product with another matrix
    ///
    /// # Arguments
    /// * `other`: A reference to the matrix to multiply with.
    ///
    /// returns: Matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use dtypes::Matrix;
    ///
    /// let m = Matrix::new(vec![vec![1.0, 2.0],
    ///                         vec![3.0, 4.0]]);
    /// let n = Matrix::new(vec![vec![5.0, 6.0],
    ///                         vec![7.0, 8.0]]);
    /// let p = m.mul(&n);
    /// assert_eq!(p.data, vec![vec![19.0, 22.0],
    ///                         vec![43.0, 50.0]]);
    /// ```
    pub fn matmul(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    // result.data[i][j] += self.data[i][k] * other.data[k][j];
                    result.set(i, j, result.get(i, j) + self.get(i, k) * other.get(k, j));
                }
            }
        }
        result
    }

    /// Computes the transpose of a matrix
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use crate::dtypes::matrix::Matrix;

    #[test]
    fn test_matmul() {
        let m = Matrix::new(vec![vec![1.0, 2.0],
                                 vec![3.0, 4.0]]);
        let n = Matrix::new(vec![vec![5.0, 6.0],
                                 vec![7.0, 8.0]]);
        let p = m.matmul(&n);
        assert_eq!(p.data, vec![vec![19.0, 22.0],
                                vec![43.0, 50.0]]);
    }

    #[test]
    fn test_get() {
        let m = Matrix::new(vec![vec![1.0, 2.0],
                                 vec![3.0, 4.0]]);
        assert_eq!(m.get(1, 1), 4.0);
    }

    #[test]
    fn test_set() {
        let mut m = Matrix::new(vec![vec![1.0, 2.0],
                                     vec![3.0, 4.0]]);
        m.set(1, 1, 69.0);
        assert_eq!(m.get(1, 1), 69.0);
    }

    #[test]
    fn test_transpose() {
        let m = Matrix::new(vec![vec![1.0, 2.0],
                                 vec![3.0, 4.0]]);
        let p = m.transpose();
        assert_eq!(p.data, vec![vec![1.0, 3.0],
                                vec![2.0, 4.0]]);
    }
}