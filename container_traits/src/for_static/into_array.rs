// IntoArray can only be implemented with one specific N.
// unfortunately we can not put N as associated constant
// because then we can not return array of length N

use super::NumberOfDegreesOfFreedom;

use crate::IntoIter;

pub trait IntoArray<T,const N:usize> : Sized+IntoIter<T>+NumberOfDegreesOfFreedom<T> {
    // compile time check for the dimensions
    const CHECK:()={
        assert!(<Self as NumberOfDegreesOfFreedom<T>>::NDOFS == N, "NDOFS must be equal to N!");
    };

    fn into_array(self) -> [T;N] {
        let _=Self::CHECK;
        utils::iter::next_chunk(
            & mut self.into_iterator()).ok().unwrap()
    }
}

impl<T,const N:usize> IntoArray<T,N> for [T;N] {}
