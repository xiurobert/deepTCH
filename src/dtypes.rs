/*
In this case a matrix is just a 2d array lol

E.g.
[
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]
*/

pub struct Matrix {
    rows: i64,
    cols: i64,
    data: Vec<Vec<f64>>,
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