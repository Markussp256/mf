use num_traits::Zero;

use crate::IndexOutOfBoundsError;

use generic_array::{ArrayLength,GenericArray};


pub trait TryPutAt<Index,T> : Sized {
    fn try_put_at(index:Index,t:T) -> Result<Self,IndexOutOfBoundsError<Index>> where T : Zero;
}

impl<T:Zero, N:ArrayLength> TryPutAt<usize,T> for GenericArray<T,N> {
    fn try_put_at(index:usize,t:T) -> Result<Self,IndexOutOfBoundsError<usize>> where T : Zero {
        IndexOutOfBoundsError::try_new(&N::to_usize(), &index)?;
        let zeros=|i:usize|std::iter::repeat_with(T::zero).take(i);
        Ok(GenericArray::try_from_iter(
            zeros(index)
             .chain(std::iter::once(t))
             .chain(zeros(N::to_usize()-index-1))).unwrap())
    }
}


impl<T:Zero, const N:usize> TryPutAt<usize,T> for [T;N] {
    fn try_put_at(index:usize,t:T) -> Result<Self,IndexOutOfBoundsError<usize>> where T : Zero {
        IndexOutOfBoundsError::try_new(&N, &index)?;
        let mut ot = Some(t);
        Ok(std::array::from_fn(|i|if i == index { ot.take().unwrap() } else { T::zero() }))
    }
}