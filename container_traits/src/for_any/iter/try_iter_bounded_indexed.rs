use crate::{DimensionMismatchError,LowerBoundUpperBoundError,IndexOutOfBoundsError};

pub trait TryTryIterBoundedIndexed<Index,T> {
    fn try_iter_bounded_indexed<'a>(&'a self, lb:Index, ub:Index) -> Result<impl ExactSizeIterator<Item=(Index,&'a T)>, DimensionMismatchError<Index>> where T:'a;
}

macro_rules! iter_impl {
    () => {
        fn try_iter_bounded_indexed<'a>(&'a self,lb:usize, ub:usize) -> Result<impl ExactSizeIterator<Item=(usize,&'a T)>, DimensionMismatchError<usize>> where T: 'a {
            LowerBoundUpperBoundError::try_new(&lb,&ub)?;
            IndexOutOfBoundsError::try_new(&self.len(),&ub)?;
            Ok(self.iter()
                   .enumerate()
                   .skip(lb)
                   .take(ub-lb+1))
        }
    };
}

impl<T> TryTryIterBoundedIndexed<usize,T> for Vec<T> {
    iter_impl!();
}

impl<T,const N:usize> TryTryIterBoundedIndexed<usize,T> for [T;N] {
    iter_impl!();
}