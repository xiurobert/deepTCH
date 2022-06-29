fn mul(x: f64, y: f64) -> f64 {
    x * y
}

fn id(x: f64) -> f64 {
    x
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn neg(x: f64) -> f64 {
    -x
}

/// Returns 1.0 if x < y else 0.0
fn lt(x: f64, y: f64) -> f64 {
    if x < y {
        1.0
    } else {
        0.0
    }
}

/// Returns 1.0 if x == y else 0.0
fn eq(a: f64, b: f64) -> f64 {
    if a == b {
        1.0
    } else {
        0.0
    }
}

/// Returns x if x > y else y
fn max(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
}

/// Checks if x and y are close (with a tolerance of 1e-2)
fn is_close(x: f64, y: f64) -> bool {
    if (x - y).abs() < 1e-2 {
        true
    } else {
        false
    }
}

/// Returns x if x > 0 else 0
fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

const EPSILON: f64 = 1e-6;

/// Returns the natural logarithm of x + EPSILON
fn log(x: f64) -> f64 {
    (x + EPSILON).ln()
}

/// Returns e^x
fn exp(x: f64) -> f64 {
    x.exp()
}

/// Derivative of natural logarithm
fn log_back(x: f64, d: f64) -> f64 {
    d * (1.0 / x)
}

/// Returns 1.0 / x
fn inv(x: f64) -> f64 {
    1.0 / x
}

/// Derivative of the reLU function
fn relu_back(x: f64, d: f64) -> f64 {
    if x > 0.0 {
        d
    } else if x < 0.0 {
        0.0
    } else {
        // by convention, since the derivative of relu is not defined at 0
        1.0
    }
}