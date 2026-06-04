use generic_array::{ArrayLength,GenericArray};


pub trait FromFn<Index,T> {
    fn from_fn(f:impl Fn(Index) -> T) -> Self;
}

impl<T,const N:usize> FromFn<usize,T> for [T;N] {
    fn from_fn(f:impl Fn(usize) -> T) -> Self {
        std::array::from_fn(f)
    }
}

impl<T,N:ArrayLength> FromFn<usize,T> for GenericArray<T,N> {
    fn from_fn(f:impl Fn(usize) -> T) -> Self {
        let n=N::to_usize();
        GenericArray::try_from_iter((0..n).into_iter().map(f)).unwrap()
    }
}