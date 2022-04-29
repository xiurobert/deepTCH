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
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        panic!("Vectors must be the same length to actually take a dot product");
    }
    let mut i = 0;
    let mut tally = 0.0;
    // easy for branch predictor
    while i < a.len() {
        tally += a[i] * b[i];
        i+= 1;
    }
    tally
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let c = super::dot_product(&a, &b);
        assert_eq!(c, 32.0);
    }
}