pub struct Tensor {
    shape: Vec<i32>,
    data: Vec<f64>
}

impl Tensor {
    pub fn new(shape: &[i32], data: &[f64]) -> Tensor {
        Tensor {
            shape: shape.to_vec(),
            data: data.to_vec()
        }
    }

    pub fn zeros(shape: &[i32]) -> Tensor {
        let mut zeros = Vec::<f64>::new();
        let mut size = 1;
        for dim in shape {
            size *= dim;
        }
        for _ in 0..size {
            zeros.push(0.0);
        }
        Tensor {
            shape: shape.to_vec(),
            data: zeros
        }
    }

    pub fn ones(shape: &[i32]) -> Tensor {
        let mut ones = Vec::<f64>::new();
        let mut size = 1;
        for dim in shape {
            size *= dim;
        }
        for _ in 0..size {
            ones.push(1.0);
        }
        Tensor {
            shape: shape.to_vec(),
            data: ones
        }
    }

    pub fn get(&self, pos: &[usize]) -> f64 {
        let mut idx = 0;
        // todo: fix this since it won't work obviously
        for (i, loc) in pos.iter().enumerate() {
            idx += loc * (i+1);
        }
        self.data[idx as usize]
    }

    pub fn set(&mut self, pos: &[i32], data: f64) {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_make_zeros() {
        let tens_zeros = super::Tensor::zeros(&[2, 3]);
        assert_eq!(tens_zeros.shape, vec![2, 3]);
        assert_eq!(tens_zeros.data, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_make_ones() {
        let tens_ones = super::Tensor::ones(&[2, 3]);
        assert_eq!(tens_ones.shape, vec![2, 3]);
        assert_eq!(tens_ones.data, vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_get_mat() {
        let tens = super::Tensor::new(&[4, 2],
                                      &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(tens.get(&[1,1 ]), 4.0);
    }

    #[test]
    fn test_get_3d() {
        let tens = super::Tensor::new(&[2,3,3],
                                      &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0]);
        assert_eq!(tens.get(&[1,2,3]), 6.0);
    }
}