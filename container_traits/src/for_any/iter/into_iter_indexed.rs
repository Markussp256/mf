
// IntoIterIndexed<Index,T> is equivalent to IntoIter<(Index,T)>
// its added for completeness


use super::IntoIter;
use generic_array::{ArrayLength, GenericArray};

pub trait IntoIterIndexed<Index,T> : Sized {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(Index,T)>;
}

impl<T> IntoIterIndexed<usize,T> for Vec<T> {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(usize,T)> {
        self.into_iterator().enumerate()
    }
}

impl<T, N:ArrayLength> IntoIterIndexed<usize,T> for GenericArray<T,N>  {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(usize,T)> {
        self.into_iterator().enumerate()
    }
}

impl<T, const N:usize> IntoIterIndexed<usize,T> for [T;N]  {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(usize,T)> {
        self.into_iterator().enumerate()
    }
}

// pub trait Iter : AsSlice {
//     fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T> {
//         self.as_slice()
//             .iter()
//     }
// }