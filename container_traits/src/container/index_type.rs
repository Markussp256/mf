use std::ops::Add;
use num_traits::Zero;
use crate::{ContainerConstructError, ContainerIndex, IntoInner, IntoIter, Iter, LenTooSmallError, TryFromIterator};
use super::index_iterator::ContainerIndexIterator;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexN<const N:usize>([usize;N]);

impl<const N:usize> Add for IndexN<N> {
    type Output=Self;
    fn add(self, rhs: Self) -> Self {
        Self(crate::array_op::add(self.0,rhs.0))
    }
}

impl<const N:usize> Iter<usize> for IndexN<N> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a usize> where Self : 'a {
        self.0
            .iter()
    }
}

impl<const N:usize> IntoInner for IndexN<N> {
    type InnerT=[usize;N];
    fn into_inner(self) -> Self::InnerT {
        self.0
    }
}

impl<const N:usize> IntoIter<usize> for IndexN<N> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=usize> {
        self.0
            .into_iter()
    }
}

impl<const N:usize> TryFromIterator<usize,ContainerConstructError<usize>> for IndexN<N> {
    fn try_take_away<I:Iterator<Item=usize>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
        utils::iter::next_chunk(iter)
            .map(|arr|Self(arr))
            .map_err(|v:Vec<usize>|LenTooSmallError::new(N,v.len()).into())
    }

    crate::try_from_iter_impl!(usize);
}

impl<const N:usize> Zero for IndexN<N> {
    fn is_zero(&self) -> bool {
        self.0
            .iter()
            .all(Zero::is_zero)
    }

    fn zero() -> Self {
        Self([0;N])
    }
}

impl<const N:usize> Default for IndexN<N> {
    fn default() -> Self {
        Self([0;N])
    }
}

impl<const N:usize> ContainerIndex for IndexN<N> {
    fn index_iterator(self) -> impl ExactSizeIterator<Item=Self> {
        ContainerIndexIterator::new_exact_size(self.0)
            .map(|ind|Self(ind))
    }
}

impl<S:Into<[usize;N]>,const N:usize> From<S> for IndexN<N> {
    fn from(value: S) -> Self {
        IndexN(value.into()) // [value.0,value.1]
    }
}