use crate::ContainerConstructError;

pub trait TryFromFn<Index,T,E=ContainerConstructError<Index>> : Sized {
    fn try_from_fn(f:impl Fn(Index) -> T) -> Result<Self,E>;
}

impl<T,const N:usize> TryFromFn<usize,T> for [T;N] {
    fn try_from_fn(f:impl Fn(usize) -> T) -> Result<Self,ContainerConstructError<usize>> {
        Ok(std::array::from_fn(f))
    }
}