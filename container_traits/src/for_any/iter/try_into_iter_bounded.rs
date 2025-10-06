use crate::{DimensionMismatchError,LowerBoundUpperBoundError,IndexOutOfBoundsError};

pub trait TryIntoTryIterBounded<Index,T> {
    fn try_into_try_iter_bounded(self, lb:Index, ub:Index) -> Result<impl ExactSizeIterator<Item=T>, DimensionMismatchError<Index>>;
}

macro_rules! iter_impl {
    () => {
        fn try_into_try_iter_bounded(self,lb:usize, ub:usize) -> Result<impl ExactSizeIterator<Item=T>, DimensionMismatchError<usize>> {
            LowerBoundUpperBoundError::try_new(&lb,&ub)?;
            IndexOutOfBoundsError::try_new(&self.len(),&ub)?;
            Ok(self.into_iter()
                   .skip(lb)
                   .take(ub-lb+1))
        }
    };
}

impl<T> TryIntoTryIterBounded<usize,T> for Vec<T> {
    iter_impl!();
}

impl<T,const N:usize> TryIntoTryIterBounded<usize,T> for [T;N] {
    iter_impl!();
}