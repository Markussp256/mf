use crate::IndexOutOfBoundsError;
use generic_array::{ArrayLength, GenericArray};

pub trait Get<Index,T> {
    fn get(&self, index:Index) -> Result<&T,IndexOutOfBoundsError<Index>>;
}

macro_rules! impl_get {
    () => {
        fn get(&self, index:usize) -> Result<&T,IndexOutOfBoundsError<usize>> {
            IndexOutOfBoundsError::try_new(&self.len(), &index)?;
            Ok(&self[index])
        }        
    };
}

impl<T> Get<usize,T> for Vec<T> {
    impl_get!();
}

impl<T, N : ArrayLength> Get<usize,T> for GenericArray<T,N> {
    impl_get!();
}

impl<T, const N:usize> Get<usize,T> for [T;N] {
    impl_get!();
}