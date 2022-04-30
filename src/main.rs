mod dtypes;
mod autodiff;

use dtypes::matrix::Matrix;

fn main() {
    println!("Hello");
    println!("{:?}", Matrix::random(2, 2));
}
