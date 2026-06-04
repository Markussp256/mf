use crate::{ContainerConstructError, FromFn};
use generic_array::{GenericArray, ArrayLength};

pub trait TryFromFn<Index,T,E=ContainerConstructError<Index>> : Sized {
    fn try_from_fn(size:Index, f:impl Fn(Index) -> T) -> Result<Self,E>;
}

impl<T> TryFromFn<usize,T> for Vec<T> {
    fn try_from_fn(len:usize, f:impl Fn(usize) -> T) -> Result<Self,ContainerConstructError<usize>> {
        Ok((0..len).map(f)
                   .collect())
    }
}

impl<T, N : ArrayLength> TryFromFn<usize,T> for GenericArray<T,N> {
    fn try_from_fn(size:usize, f:impl Fn(usize) -> T) -> Result<Self,ContainerConstructError<usize>> {
        Ok(<Self as FromFn<usize,T>>::from_fn(size,f))
    }
}

impl<T,const N:usize> TryFromFn<usize,T> for [T;N] {
    fn try_from_fn(_:usize, f:impl Fn(usize) -> T) -> Result<Self,ContainerConstructError<usize>> {
        Ok(std::array::from_fn(f))
    }
}