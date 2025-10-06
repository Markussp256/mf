use crate::{DimensionMismatchError,LowerBoundUpperBoundError,IndexOutOfBoundsError};

pub trait TryTryIntoTryTryIterBoundedIndexed<Index,T> {
    fn try_into_try_iter_bounded_indexed(self, lb:Index, ub:Index) -> Result<impl ExactSizeIterator<Item=(Index,T)>, DimensionMismatchError<Index>>;
}

macro_rules! iter_impl {
    () => {
        fn try_into_try_iter_bounded_indexed(self,lb:usize, ub:usize) -> Result<impl ExactSizeIterator<Item=(usize,T)>, DimensionMismatchError<usize>> {
            LowerBoundUpperBoundError::try_new(&lb,&ub)?;
            IndexOutOfBoundsError::try_new(&self.len(),&ub)?;
            Ok(self.into_iter()
                   .enumerate()
                   .skip(lb)
                   .take(ub-lb+1))
        }
    };
}

impl<T> TryTryIntoTryTryIterBoundedIndexed<usize,T> for Vec<T> {
    iter_impl!();
}

impl<T,const N:usize> TryTryIntoTryTryIterBoundedIndexed<usize,T> for [T;N] {
    iter_impl!();
}