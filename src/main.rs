mod dtypes;

fn main() {
    println!("Dot product of 1 2 3 and 4 5 6: {}",
             dtypes::dot_product(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]));
}
