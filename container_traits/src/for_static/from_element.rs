pub trait FromElement<T:Clone> {
    fn from_element(t:T) -> Self;
}

impl<T:Clone,const N:usize> FromElement<T> for [T;N] {
    fn from_element(t:T) -> Self {
        std::array::from_fn(|_|t.clone())
    }
}