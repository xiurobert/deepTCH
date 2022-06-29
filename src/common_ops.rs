use std::error::Error;

/// Negates every element in the list
fn negate_list(ls: Vec<f64>) -> Vec<f64> {
    ls.iter().map(|x| -x).collect()
}

/// Elementwise addition of two lists
///
/// # Examples
///
/// ```
///
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![4.0, 5.0, 6.0];
/// let c = add_lists(a, b).unwrap();
///
/// assert_eq!(c, vec![5.0, 7.0, 9.0]);
/// ```
fn add_lists(ls1: Vec<f64>, ls2: Vec<f64>) -> Result<Vec<f64>, String> {
    if ls1.len() != ls2.len() {
        return Err(String::from("Lists must be the same length"));
    }
    Ok(ls1.iter().zip(ls2.iter()).map(|(x, y)| x + y).collect())
}

/// Elementwise summation of a list
fn sum_list(ls: Vec<f64>) -> f64 {
    ls.iter().sum()
}

/// Product of an entire list
fn product_list(ls: Vec<f64>) -> f64 {
    ls.iter().product()
}