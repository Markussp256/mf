use super::OCTSize;

use generic_array::{GenericArray, ArrayLength};

pub trait FromElement<Index,T:Clone> : Sized {
    fn from_element(size:Index,t:T) -> Self;
}

impl<T:Clone> FromElement<usize,T> for Vec<T> {
    fn from_element(len:usize,t:T) -> Self {
        std::iter::repeat(t)
            .take(len)
            .collect()
    }
}

impl<T:Clone, N : ArrayLength> FromElement<usize,T> for GenericArray<T,N> {
    fn from_element(size:usize,t:T) -> Self {
        GenericArray::try_from_iter(
            std::iter::repeat(t)
                .take(size)).unwrap()
    }
}

impl<T:Clone, const N:usize> FromElement<usize,T> for [T;N] {
    fn from_element(_:usize,t:T) -> Self {
        std::array::from_fn(|_|t.clone())
    }
}

impl<Index:Copy+PartialEq,T:Clone,S:FromElement<Index,T>+OCTSize<Index>, const N:usize> FromElement<(usize,Index),T> for [S;N] {
    fn from_element(size:(usize,Index),t:T) -> Self {
        std::array::from_fn(|_|S::from_element(size.1, t.clone()))
    }
}