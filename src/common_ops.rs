
/// Negates every element in the list
fn negate_list(ls: Vec<f64>) -> Vec<f64> {
    ls.iter().map(|x| -x).collect()
}

/// Elementwise addition of two lists
pub fn add_lists(ls1: Vec<f64>, ls2: Vec<f64>) -> Result<Vec<f64>, String> {
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

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use crate::math_ops::is_close;
    use super::*;

    fn cmp_list_is_close(ls1: Vec<f64>, ls2: Vec<f64>) -> bool {
        ls1.iter().zip(ls2.iter()).all(|(x, y)| is_close(*x, *y))
    }
    proptest! {
        #[test]
        fn test_negate_list(ls in prop::collection::vec(prop::num::f64::NORMAL, 0..100)) {
            let out_ls: Vec<f64> = ls.iter().map(|x| -x).collect();
            assert!(cmp_list_is_close(negate_list(ls), out_ls));
        }

        #[test]
        fn test_add_lists(ls1 in prop::collection::vec(prop::num::f64::NORMAL, 0..100)) {
            let ls2 = ls1.clone();
            let ls1_copy = ls1.clone();
            let ls2_copy = ls2.clone();
            if let Ok(out_ls) = add_lists(ls1, ls2) {
                assert!(cmp_list_is_close(out_ls, ls1_copy.iter().zip(ls2_copy.iter()).map(|(x, y)| x + y).collect()));
            } else {
                panic!("Why");
            }
        }

        #[test]
        fn test_sum_list(ls in prop::collection::vec(prop::num::f64::NORMAL, 0..100)) {
            let ls_copy = ls.clone();
            assert_eq!(sum_list(ls), ls_copy.iter().sum());
        }

        #[test]
        fn test_product_list(ls in prop::collection::vec(prop::num::f64::NORMAL, 0..100)) {
            let ls_copy = ls.clone();
            assert_eq!(product_list(ls), ls_copy.iter().product());
        }
    }
}