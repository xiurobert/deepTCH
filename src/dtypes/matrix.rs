pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
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
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }
}