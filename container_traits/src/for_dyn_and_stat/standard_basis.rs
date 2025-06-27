use num_traits::{Zero,One};
use crate::IndexOutOfBoundsError;

use super::TryPutAt;

pub trait StandardBasis : Sized {
    fn try_standard_basis_element(len:usize, index:usize) -> Result<Self,IndexOutOfBoundsError<usize>>;


    fn standard_basis(len:usize) -> impl ExactSizeIterator<Item=Self> {
        (0..len).map(move |i|Self::try_standard_basis_element(len, i).unwrap())
    }
}

// for UnitVectors we will implement StandardBasis but not TryPutAt.
// therefore we can not implement StandardBasis whenever we have TryPutAt.

impl<T:Zero+One> StandardBasis for Vec<T> {
    fn try_standard_basis_element(len:usize, index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
        <Self as TryPutAt<usize,T>>::try_put_at(len, index,T::one())
    }
}