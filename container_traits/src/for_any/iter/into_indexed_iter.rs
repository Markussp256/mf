
// IndexedIntoIter<Index,T> is equivalent to IntoIter<(Index,T)>
// its added for completeness

use super::IntoIter;

pub trait IntoIndexedIter<Index,T> : Sized {
    fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(Index,T)>;
}

impl<T> IntoIndexedIter<usize,T> for Vec<T> {
    fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(usize,T)> {
        self.into_iterator().enumerate()
    }
}
impl<T, const N:usize> IntoIndexedIter<usize,T> for [T;N]  {
    fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(usize,T)> {
        self.into_iterator().enumerate()
    }
}

// pub trait Iter : AsSlice {
//     fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T> {
//         self.as_slice()
//             .iter()
//     }
// }