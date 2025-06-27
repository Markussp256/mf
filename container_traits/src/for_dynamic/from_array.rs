use super::FromVec;

pub trait FromArray<T> {
    fn from_array<const N:usize>(array:[T;N]) -> Self;
}

impl<T,S:FromVec<T>> FromArray<T> for S {
    fn from_array<const N:usize>(array:[T;N]) -> Self {
        Self::from_vec(array.into())
    }
}