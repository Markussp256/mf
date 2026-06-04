use generic_array::{ArrayLength, GenericArray};

pub trait ItemT {
    type T;
}

impl<T> ItemT for Vec<T> {
    type T=T;
}

impl<T, N:ArrayLength> ItemT for GenericArray<T,N> {
    type T=T;
}

impl<T, const N:usize> ItemT for [T;N] {
    type T=T;
}