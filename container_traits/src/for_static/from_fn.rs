pub trait FromFn<Index,T> {
    fn from_fn(f:impl Fn(Index) -> T) -> Self;
}

impl<T,const N:usize> FromFn<usize,T> for [T;N] {
    fn from_fn(f:impl Fn(usize) -> T) -> Self {
        std::array::from_fn(f)
    }
}