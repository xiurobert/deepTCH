pub fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    let mut result = Vec::new();
    let step = (end - start) / (n as f64 - 1.0);
    for i in 0..n {
        result.push(start + i as f64 * step);
    }
    result
}

mod test {
    use super::*;

    #[test]
    fn test_linspace() {
        let result = linspace(0.0, 1.0, 5);
        assert_eq!(result, vec![0.0, 0.25, 0.50, 0.75, 1.0]);
    }

    // #[test]
    // fn test_logspace() {
    //     let result = logspace(0.0, 4.0, 5, 2.0);
    //     assert_eq!(result, vec![1.0, 2.0, 4.0, 8.0, 16.0]);
    // }
}