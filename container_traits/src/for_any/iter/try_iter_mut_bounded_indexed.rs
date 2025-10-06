use crate::{DimensionMismatchError, IndexOutOfBoundsError, LowerBoundUpperBoundError};

pub trait TryTryIterMutBoundedIndexed<Index,T> {
    fn try_iter_mut_bounded_indexed<'a>(&'a mut self, lb:Index, ub:Index) -> Result<impl ExactSizeIterator<Item=(Index,&'a mut T)>, DimensionMismatchError<Index>> where T : 'a;
}

macro_rules! iter_impl {
    () => {
        fn try_iter_mut_bounded_indexed<'a>(&'a mut self,lb:usize, ub:usize) -> Result<impl ExactSizeIterator<Item=(usize,&'a mut T)>, DimensionMismatchError<usize>> where T : 'a {
            LowerBoundUpperBoundError::try_new(&lb,&ub)?;
            IndexOutOfBoundsError::try_new(&self.len(),&ub)?;
            Ok(self.iter_mut()
                   .enumerate()
                   .skip(lb)
                   .take(ub-lb+1))
        }
    };
}

impl<T> TryTryIterMutBoundedIndexed<usize,T> for Vec<T> {
    iter_impl!();
}

impl<T,const N:usize> TryTryIterMutBoundedIndexed<usize,T> for [T;N] {
    iter_impl!();
}