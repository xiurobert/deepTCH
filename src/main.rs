mod dtypes;
mod autodiff;

use crate::autodiff::variable::Variable;
use crate::dtypes::vector::{dot_product};


fn main() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![4.0, 5.0, 6.0];
    println!("Dot product of 1 2 3 and 4 5 6: {}",
             dot_product(&a, &b));
    let a_var = Variable::new_no_gradients(3.0);
    let b_var = Variable::new_no_gradients(5.0);
    println!("{:?}", a_var * b_var);
    println!("{:?}", Variable::new_no_gradients(9.0) / Variable::new_no_gradients(3.0));
}
