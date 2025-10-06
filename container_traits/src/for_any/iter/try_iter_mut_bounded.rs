use crate::{DimensionMismatchError, IndexOutOfBoundsError, LowerBoundUpperBoundError};


pub trait TryIterMutBounded<Index,T> {
    fn try_iter_mut_bounded<'a>(&'a mut self,lb:Index,ub:Index) -> Result<impl ExactSizeIterator<Item=&'a mut T>,DimensionMismatchError<Index>>
    where T:'a;
}


macro_rules! try_iter_bounded_impl {
    () => {
        fn try_iter_mut_bounded<'a>(&'a mut self,lb:usize,ub:usize) -> Result<impl ExactSizeIterator<Item=&'a mut T>,DimensionMismatchError<usize>> where T:'a {
            LowerBoundUpperBoundError::try_new(&lb,&ub)?;
            IndexOutOfBoundsError::try_new(&self.len(),&ub)?;
            Ok(self[lb..=ub]
                 .iter_mut())
        }
    };
}

impl<T:> TryIterMutBounded<usize,T> for Vec<T> {
    try_iter_bounded_impl!();
}

impl<T, const N:usize> TryIterMutBounded<usize,T> for [T;N] {
    try_iter_bounded_impl!();
}