use crate::{ContainerConstructError, SizeNotEqualToRequiredSizeError};
use generic_array::{ArrayLength,GenericArray};



pub trait TryFromVec<T,E> : Sized {
    fn try_from_vec(v:Vec<T>) -> Result<Self,E>;
}

impl<T, const N:usize> TryFromVec<T,ContainerConstructError<usize>> for [T;N] {
    fn try_from_vec(v:Vec<T>) -> Result<Self,ContainerConstructError<usize>> {
        v.try_into()
         .map_err(|v:Vec<T>|SizeNotEqualToRequiredSizeError::new(N,v.len()).into())
    }
}

impl<T, N:ArrayLength> TryFromVec<T,ContainerConstructError<usize>> for GenericArray<T,N> {
    fn try_from_vec(v:Vec<T>) -> Result<Self,ContainerConstructError<usize>> {
        SizeNotEqualToRequiredSizeError::try_new(N::to_usize(),v.len())?;
        Ok(GenericArray::try_from_iter(
            v.into_iter()).unwrap())
    }
}