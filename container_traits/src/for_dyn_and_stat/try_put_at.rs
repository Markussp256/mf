use num_traits::Zero;
use crate::IndexOutOfBoundsError;

use super::AnyFromIterator;

pub trait TryPutAt<Index,T> : Sized {
    fn try_put_at(size:Index,index:Index,t:T) -> Result<Self,IndexOutOfBoundsError<Index>> where T : Zero;
}

impl<T> TryPutAt<usize,T> for Vec<T> {
    fn try_put_at(len:usize,index:usize,t:T) -> Result<Self,IndexOutOfBoundsError<usize>> where T : Zero {
        IndexOutOfBoundsError::try_new(&len, &index)?;
        let z=std::iter::repeat_with(T::zero);
        Ok(z.take(index)
            .chain(std::iter::once(t))
            .chain(z.take(len-index-1))
            .collect())
    }
}


impl<T, const N:usize> TryPutAt<usize,T> for [T;N] {
    fn try_put_at(len:usize,index:usize,t:T) -> Result<Self,IndexOutOfBoundsError<usize>> where T : Zero {
        assert_eq!(len,N);
        IndexOutOfBoundsError::try_new(&len, &index)?;
        let z=std::iter::repeat_with(T::zero);
        Ok(Self::any_from_iter(None,
        z.take(index)
              .chain(std::iter::once(t))
              .chain(z.take(len-index-1))).unwrap())
    }
}