pub fn mul(x: f64, y: f64) -> f64 {
    x * y
}

pub fn id(x: f64) -> f64 {
    x
}

pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

pub fn neg(x: f64) -> f64 {
    -x
}

/// Returns 1.0 if x < y else 0.0
pub fn lt(x: f64, y: f64) -> f64 {
    if x < y {
        1.0
    } else {
        0.0
    }
}

/// Returns 1.0 if x == y else 0.0
pub fn eq(a: f64, b: f64) -> f64 {
    if a == b {
        1.0
    } else {
        0.0
    }
}

/// Returns x if x > y else y
pub fn max(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
}

/// Checks if x and y are close (with a tolerance of 1e-2)
pub fn is_close(x: f64, y: f64) -> bool {
    (x - y).abs() < 1e-2
}

/// Returns x if x > 0 else 0
pub fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

const EPSILON: f64 = 1e-6;

/// Returns the natural logarithm of x + EPSILON
pub fn log(x: f64) -> f64 {
    (x + EPSILON).ln()
}

/// Returns e^x
pub fn exp(x: f64) -> f64 {
    x.exp()
}

/// Derivative of natural logarithm
pub fn log_back(x: f64, d: f64) -> f64 {
    d * (1.0 / x)
}

/// Returns 1.0 / x
pub fn inv(x: f64) -> f64 {
    1.0 / x
}

/// Derivative of the reLU function
pub fn relu_back(x: f64, d: f64) -> f64 {
    if x > 0.0 {
        d
    } else if x < 0.0 {
        0.0
    } else {
        // by convention, since the derivative of relu is not defined at 0
        1.0
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::*;

    // Note: usually, we should NEVER compare floats with ==.
    // However, it's actually beneficial here, since we do not
    // want our implementation to deviate from the rust one.
    proptest! {
        #[test]
        fn test_add(x in 0.0..100.0, y in 0.0..100.0) {
            assert_eq!(add(x, y), x + y);
        }

        #[test]
        fn test_mul(x in 0.0..100.0, y in 0.0..100.0) {
            assert_eq!(mul(x, y), x * y);
        }

        #[test]
        fn test_id(x in 0.0..100.0) {
            assert_eq!(id(x), x);
        }

        #[test]
        fn test_neg(x in 0.0..100.0) {
            assert_eq!(neg(x), -x);
        }

        #[test]
        fn test_lt(x in 0.0..100.0, y in 0.0..100.0) {
            assert_eq!(lt(x, y), if x < y { 1.0 } else { 0.0 });
        }

        #[test]
        fn test_eq(x in 0.0..100.0, y in 0.0..100.0) {
            assert_eq!(eq(x, y), if x == y { 1.0 } else { 0.0 });
        }

        #[test]
        fn test_max(x in 0.0..100.0, y in 0.0..100.0) {
            assert_eq!(max(x, y), if x > y { x } else { y });
        }

        #[test]
        fn test_is_close(x in 0.0..100.0, y in 0.0..100.0) {
            assert_eq!(is_close(x, y), (x - y).abs() < 1e-2);
        }

        #[test]
        fn test_relu(x in 0.0..100.0) {
            assert_eq!(relu(x), if x > 0.0 { x } else { 0.0 });
        }

        #[test]
        fn test_log(x in 0.0..100.0) {
            assert_eq!(log(x), (x + EPSILON).ln());
        }

        #[test]
        fn test_exp(x in 0.0..100.0) {
            assert_eq!(exp(x), x.exp());
        }

        #[test]
        fn test_log_back(x in 0.0..100.0, y in 0.0..100.0) {
            assert_eq!(log_back(x, y), y * (1.0 / x));
        }

        #[test]
        fn test_inv(x in 0.0..100.0) {
            assert_eq!(inv(x), 1.0 / x);
        }

        #[test]
        fn test_relu_back(x in 0.0..100.0, y in 0.0..100.0) {
            assert_eq!(relu_back(x, y), if x > 0.0 { y } else if x < 0.0 { 0.0 } else { 1.0 });
        }
    }
}