use crate::IndexOutOfBoundsError;

use generic_array::{ArrayLength, GenericArray};


pub trait TryIntoElement<Index,T> : Sized {
    fn try_into_element(self,index:Index) -> Result<T,IndexOutOfBoundsError<Index>>;
}

macro_rules! impl_try_into_element {
    () => {
        fn try_into_element(self,index:usize) -> Result<T,IndexOutOfBoundsError<usize>> {
            IndexOutOfBoundsError::try_new(&self.len(),&index)?;
            Ok(self.into_iter()
                  .nth(index)
                  .unwrap())
        }
    }
}

impl<T> TryIntoElement<usize,T> for Vec<T> {
    impl_try_into_element!();
}

impl<T,N : ArrayLength> TryIntoElement<usize,T> for GenericArray<T,N> {
    impl_try_into_element!();
}

impl<T,const N:usize> TryIntoElement<usize,T> for [T;N] {
    impl_try_into_element!();
}