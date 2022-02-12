/*
In this case a matrix is just a 2d array lol

E.g.
[
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]
*/

use std::fmt;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    /// The internal data of the matrix. This is stored as a Vec of Vecs (aka list of lists)
    data: Vec<Vec<f64>>,
}

impl Matrix {
    /// Creates a new matrix from a Vec of Vecs.
    /// This checks the data for validity and returns an error if it is invalid
    /// This is the only way to create a new matrix
    /// # Example
    /// ```
    /// use dtypes::Matrix;
    /// let m = Matrix::new(vec![
    /// vec![1.0, 2.0, 3.0],
    /// vec![4.0, 5.0, 6.0],
    /// vec![7.0, 8.0, 9.0]
    /// ]); // makes a 3x3 matrix containing 1 to 9
    ///
    /// assert_eq!(m.rows, 3);
    /// assert_eq!(m.cols, 3);
    /// ```
    pub fn new(data: Vec<Vec<f64>>) -> Result<Matrix, String> {
        let rows = data.len();
        let cols = data[0].len();
        for row in data.iter() {
            if row.len() != cols {
                return Err("All rows must be the same length".to_string());
            }
        }

        Ok(Matrix { rows, cols, data, })
    }

    /// Simply initializes an empty matrix with the given dimensions
    /// # Example
    /// ```
    /// use dtypes::Matrix;
    /// let m = Matrix::empty(3, 3); // makes a 3x3 matrix containing 0 to 0
    /// ```
    pub fn new_empty(rows: usize, cols: usize) -> Matrix {
        Matrix { rows, cols, data: vec![vec![0.0; cols]; rows], }
    }

    /// Evaluates the dot product of 2 matrices
    /// # Example
    /// ```
    /// use dtypes::Matrix;
    /// let m1 = Matrix::new(vec![
    /// vec![1.0, 2.0, 3.0],
    /// vec![4.0, 5.0, 6.0],
    /// ]);
    /// let m2 = Matrix::new(vec![
    /// vec![7.0, 8.0],
    /// vec![9.0, 10.0],
    /// vec![11.0, 12.0],
    /// ]);
    /// let m3 = m1.dot(&m2);
    /// assert_eq!(m3.rows, 2);
    /// assert_eq!(m3.cols, 2);
    /// assert_eq!(m3.data, vec![
    /// vec![58.0, 64.0],
    /// vec![139.0, 154.0]
    /// ]);
    ///
    pub fn dot(&self, other: Matrix) -> Result<Matrix, String> {
        if self.rows != other.cols || self.cols != other.rows {
            return Err(
                format!("Cannot dot a [{} by {}] matrix with a [{} by {}] matrix (Shape error)",
                               self.rows,
                               self.cols,
                               other.rows,
                               other.cols));
        }
        Ok(Matrix {
            rows: self.rows,
            cols: other.cols,
            data: vec![vec![0.0; other.cols]; self.rows],
        })

    }

    pub fn dot_vector(&self, other: Vec<f64>) {

    }

    /// Returns the transpose of the matrix
    /// This is the equivalent of evaluating np.T(m) in python
    /// # Example
    /// ```
    /// use dtypes::Matrix;
    /// let m = Matrix::new(vec![
    /// vec![1.0, 2.0, 3.0],
    /// vec![4.0, 5.0, 6.0],
    /// ]);
    /// let m_t = m.transpose();
    /// assert_eq!(m_t.rows, 3);
    /// assert_eq!(m_t.cols, 2);
    /// assert_eq!(m_t.data, vec![
    /// vec![1.0, 4.0],
    /// vec![2.0, 5.0],
    /// vec![3.0, 6.0]
    /// ]);
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new_empty(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix {{ rows: {}, cols: {}, data: {:?} }}", self.rows, self.cols, self.data)
    }
}

/// This function simply evaluates the dot product of 2 vectors
///
/// # Example
/// vector a | vector b
/// 1      | 4
/// 2      | 5
/// 3      | 6
/// dot product gives: 1*4 + 2*5 + 3*6 = 32
///
/// ```
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![4.0, 5.0, 6.0];
/// let c = dot_product(&a, &b);
///
pub fn dot_product(a: Vec<f64>, b: Vec<f64>) -> f64 {
    if a.len() != b.len() {
        panic!("Vectors must be the same length to actually take a dot product");
    }
    let mut i = 0;
    let mut tally = 0.0;
    while i < a.len() {
        tally += a[i] * b[i];
        i+= 1;
    }
    tally
}