use crate::{ContainerConstructError, SizeNotEqualToRequiredSizeError};

pub trait TryFromVec<T,E> : Sized {
    fn try_from_vec(v:Vec<T>) -> Result<Self,E>;
}

impl<T, const N:usize> TryFromVec<T,ContainerConstructError<usize>> for [T;N] {
    fn try_from_vec(v:Vec<T>) -> Result<Self,ContainerConstructError<usize>> {
        v.try_into()
         .map_err(|v:Vec<T>|SizeNotEqualToRequiredSizeError::new(N,v.len()).into())
    }
}