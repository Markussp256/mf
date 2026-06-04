use generic_array::{ArrayLength, GenericArray};

use crate::EmptyContainerError;

pub trait First<T> {
    fn first(&self) -> Result<&T,EmptyContainerError>;
}

macro_rules! impl_first {
    () => {
        fn first(&self) -> Result<&T,EmptyContainerError> {
            self.as_slice()
                .first()
                .ok_or(EmptyContainerError)
        }
    };
}

impl<T> First<T> for Vec<T> {
    impl_first!();
}

impl<T,N:ArrayLength> First<T> for GenericArray<T,N> {
    impl_first!();
}

impl<T,const N:usize> First<T> for [T;N] {
    impl_first!();
}

#[macro_export]
macro_rules! first_from_get {
    () => {
        fn first(&self) -> Result<& T,$crate::EmptyContainerError> {
            self.get(Index::try_from_iter(std::iter::repeat(0).take(Index::DIM)).unwrap())
                .map_err(|_|$crate::EmptyContainerError)
        }
    };
}