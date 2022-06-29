use crate::autodiff::Variable;

struct Scalar {
    value: f64,
}

/// Computes an approximation of the derivative of the scalar function f with respect to 1 arg
///
/// # Arguments
/// * `f` - The scalar function to differentiate (a function that accepts some list of numbers and returns a number)
/// * `values` - The values that the function accepts
/// * `arg` - The index of the argument to differentiate with respect to
/// * `eps` - The step size to use when approximating the derivative. Usually 1e-6.
fn central_difference(f: &dyn Fn(&[f64]) -> f64, values: &Vec<f64>, arg: usize, eps: &f64) -> Result<f64, String> {
    if arg >= values.len() {
        return Err(String::from("Argument out of bounds"));
    }

    let mut v_cl = values.clone();
    v_cl[arg] -= eps;
    let initial = f(&v_cl);
    v_cl[arg] += 2.0 * eps;
    let result = f(&v_cl);
    Ok((result - initial) / (2.0 * eps))
}