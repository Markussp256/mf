use num_traits::{Zero,One};
use crate::IndexOutOfBoundsError;

use super::TryPutAt;

use generic_array::{ArrayLength,GenericArray};


pub trait StandardBasis : Sized {
    fn try_standard_basis_element(index:usize) -> Result<Self,IndexOutOfBoundsError<usize>>;

    fn standard_basis() -> impl ExactSizeIterator<Item=Self>;
}

// for UnitVectors we will implement StandardBasis but not TryPutAt.
// therefore we can not implement StandardBasis whenever we have TryPutAt.

impl<T:Zero+One, const N:usize> StandardBasis for [T;N] {
    fn try_standard_basis_element(index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
        <Self as TryPutAt<usize,T>>::try_put_at(index,T::one())
    }

    fn standard_basis() -> impl ExactSizeIterator<Item=Self> {
        (0..N).map(|i|Self::try_standard_basis_element(i).unwrap())
    }
}

impl<T:Zero+One, N:ArrayLength> StandardBasis for GenericArray<T,N> {
    fn try_standard_basis_element(index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
        <Self as TryPutAt<usize,T>>::try_put_at(index,T::one())
    }

    fn standard_basis() -> impl ExactSizeIterator<Item=Self> {
        let n=N::to_usize();
        (0..n).map(|i|Self::try_standard_basis_element(i).unwrap())
    }
}