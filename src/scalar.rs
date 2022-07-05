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
pub fn central_difference(f: &dyn Fn(&[f64]) -> f64, values: &Vec<f64>, arg: usize, eps: &f64) -> Result<f64, String> {
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

struct Context;

trait AutoDifferentiable {
    fn forward(&self, ctx: Context, values: &Vec<f64>) -> Result<f64, String>;
    fn backward(&self, ctx: Context, values: &Vec<f64>, grad_output: f64) -> Result<Vec<f64>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use crate::math_ops::is_close;
    proptest! {
        #[test]
        fn test_central_difference(vals in proptest::collection::vec(prop::num::f64::NORMAL, 1..100)) {
            let eps = 1e-6;
            let arg = rand::thread_rng().gen_range(0..vals.len());
            let f = |x: &[f64]| x[arg];
            let vals_copy = vals.clone();
            if let Ok(result) = central_difference(&f, &vals, arg, &eps) {
                let mut vals_copy = vals_copy;
                vals_copy[arg] -= eps;
                let initial = f(&vals_copy);
                vals_copy[arg] += 2.0 * eps;
                let res2 = f(&vals_copy);
                let expected = (res2 - initial) / (2.0 * eps);
                assert!(is_close(result, expected));
            } else {
                panic!("Why");
            }
        }


    }
}