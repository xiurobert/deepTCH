/// Tensor library
struct Tensor<T> {
    data: Vec<T>,
    shape: Vec<usize>,
    strides: Vec<usize>
}

/// Computes the Tensor strides from the given shape
fn strides_from_shape(shape: &[usize]) -> Vec<usize> {
    let mut strides = vec![1];
    for i in 1..shape.len() {
        strides.push(strides[i - 1] * shape[i - 1]);
    }
    strides
}

/// Converts some Tensor index to the actual position in memory
/// For example, suppose you have a Matrix with shape [2, 3] and strides [6, 1].
/// Then the index [1, 2] would be converted to 6*1 + 2*1 = 8.
///
/// In general, if we have some indexes `[i_1, i_2, ..., i_n]`, and some strides `[s_1, s_2, ..., s_n]`,
/// then the index is computed as:
/// `index = s_1 * i_1 + s_2 * i_2 + ... + s_n * i_n`
/// where s_i is the stride of the ith dimension.
///
/// # Arguments
/// * `index` - The index into the tensor
/// * `strides` - The strides of the tensor
///
///
/// # Errors
/// If `index` and `strides` are not the same length
fn index_to_position(index: &[usize], strides: &[usize]) -> Result<usize, ()> {
    if index.len() != strides.len() {
        return Err(());
    }
    Ok(index.iter().zip(strides.iter()).fold(0, |acc, (i, s)| acc + i * s))
}

/// Converts a position in memory to an index into the Tensor
/// Is the reverse of [`index_to_position`]
fn position_to_index(position: usize, shape: &[usize]) -> Vec<usize> {
    let strides = strides_from_shape(shape);
    let mut position = position;
    let mut index = vec![0; shape.len()];
    for i in 0..shape.len() {
        index[i] = position / strides[i];
        position %= strides[i];
    }
    index
}

impl Tensor<f64> {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_index_to_position() {
        let index = [1, 2];
        let strides = [6, 1];
        assert_eq!(8, index_to_position(&index, &strides).unwrap());

        let index = [1, 2, 3];
        let strides = [6, 1, 1];
        assert_eq!(11, index_to_position(&index, &strides).unwrap());

        let index = [69, 420];
        let strides = [42, 37];
        assert_eq!(69 * 42 + 420 * 37, index_to_position(&index, &strides).unwrap());
    }

    #[test]
    fn test_position_to_index() {
        let position = 8;
        let shape = [2, 3];
        assert_eq!(vec![1, 2], position_to_index(position, &shape));

        let position = 11;
        let shape = [2, 3];
        assert_eq!(vec![1, 2, 3], position_to_index(position, &shape));
    }
}

