use crate::dtypes::Matrix;

mod dtypes;

fn main() {
    println!("Dot product of 1 2 3 and 4 5 6: {}",
             dtypes::dot_product(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]));
    let m = Matrix::new(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0]]).unwrap();
    let m_transposed = Matrix::new(vec![
        vec![1.0, 4.0],
        vec![2.0, 5.0],
        vec![3.0, 6.0]]).unwrap();
    println!("Matrix: {:?}", &m);
    assert_eq!(m_transposed, dtypes::Matrix::transpose(&m));

}
