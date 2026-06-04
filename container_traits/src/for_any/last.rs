use crate::EmptyContainerError;

use generic_array::{ArrayLength, GenericArray};


pub trait Last<T> {
    fn last(&self) -> Result<&T,EmptyContainerError>; 
}

macro_rules! impl_last {
    () => {
        fn last(&self) -> Result<&T,EmptyContainerError> {
            self.as_slice()
                .last()
                .ok_or(EmptyContainerError)
        }
    };
}

impl<T> Last<T> for Vec<T> {
    impl_last!();
}

impl<T,N:ArrayLength> Last<T> for GenericArray<T,N> {
    impl_last!();
}

impl<T,const N:usize> Last<T> for [T;N] {
    impl_last!();
}

#[macro_export]
macro_rules! last_from_get {
    () => {
        fn last(&self) -> Result<& T,$crate::EmptyContainerError> {
            let sz=self.size();
            if sz.iter().any(|szi|szi == &0) {
                return Err($crate::EmptyContainerError);
            }
            let index=Index::try_from_iter(sz.into_iterator().map(|szi|szi-1)).unwrap();
            Ok(self.get(index).unwrap())
        }
    };
}