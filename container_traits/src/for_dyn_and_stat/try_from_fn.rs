use crate::ContainerConstructError;

pub trait TryFromFn<Index,T,E=ContainerConstructError<Index>> : Sized {
    fn try_from_fn(size:Index, f:impl Fn(Index) -> T) -> Result<Self,E>;
}

impl<T> TryFromFn<usize,T> for Vec<T> {
    fn try_from_fn(len:usize, f:impl Fn(usize) -> T) -> Result<Self,ContainerConstructError<usize>> {
        Ok((0..len).map(f)
                   .collect())
    }
}

impl<T,const N:usize> TryFromFn<usize,T> for [T;N] {
    fn try_from_fn(_:usize, f:impl Fn(usize) -> T) -> Result<Self,ContainerConstructError<usize>> {
        Ok(std::array::from_fn(f))
    }
}