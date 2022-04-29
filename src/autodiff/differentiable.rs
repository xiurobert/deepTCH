pub trait Differentiable<T> {
    fn differentiate(&self) -> T;
}

