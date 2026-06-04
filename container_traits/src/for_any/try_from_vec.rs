use crate::{ContainerConstructError, SizeNotEqualToRequiredSizeError};

use generic_array::{ArrayLength, GenericArray};


pub trait TryFromVec<T,E> : Sized {
    fn try_from_vec(v:Vec<T>) -> Result<Self,E>;
}

impl<T> TryFromVec<T,ContainerConstructError<usize>> for Vec<T> {
    fn try_from_vec(v:Vec<T>) -> Result<Self,ContainerConstructError<usize>> {
        Ok(v)
    }
}

impl<T, N:ArrayLength> TryFromVec<T,ContainerConstructError<usize>> for GenericArray<T,N> {
    fn try_from_vec(v:Vec<T>) -> Result<Self,ContainerConstructError<usize>> {
        let len=v.len();
        SizeNotEqualToRequiredSizeError::try_new(N::to_usize(),len)?;
        Ok(GenericArray::try_from_iter(v.into_iter()).unwrap())
    }
}

impl<T, const N:usize> TryFromVec<T,ContainerConstructError<usize>> for [T;N] {
    fn try_from_vec(v:Vec<T>) -> Result<Self,ContainerConstructError<usize>> {
        let len=v.len();
        v.try_into()
         .map_err(|_|SizeNotEqualToRequiredSizeError::new(N,len).into())
    }
}