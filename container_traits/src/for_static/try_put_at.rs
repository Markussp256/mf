use num_traits::Zero;

use crate::IndexOutOfBoundsError;

pub trait TryPutAt<Index,T> : Sized {
    fn try_put_at(index:Index,t:T) -> Result<Self,IndexOutOfBoundsError<Index>> where T : Zero;
}


impl<T:Zero, const N:usize> TryPutAt<usize,T> for [T;N] {
    fn try_put_at(index:usize,t:T) -> Result<Self,IndexOutOfBoundsError<usize>> where T : Zero {
        IndexOutOfBoundsError::try_new(&N, &index)?;
        let z=std::iter::repeat_with(T::zero);
        let v:Vec<T>=
            z.take(index)
             .chain(std::iter::once(t))
             .chain(z.take(N-index-1))
             .collect();
        Ok(v.try_into().ok().unwrap())
    }
}