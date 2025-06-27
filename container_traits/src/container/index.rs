use std::ops::Add;
use num_traits::Zero;
use std::fmt::Debug;
use crate::{ContainerConstructError, LenTooSmallError, IntoIter, IntoProduct, Iter, TryFromIterator};

pub trait ContainerIndex
    : Debug
     +Default
     +Clone
     +PartialOrd
     +Ord
     +Iter<usize>
     +IntoIter<usize>
     +TryFromIterator<usize,ContainerConstructError<usize>> {

    fn index_iterator(self) -> impl ExactSizeIterator<Item=Self>;

    fn is_elem_wise_op(&self,rhs:&Self, f:impl Fn(&usize,&usize) -> bool) -> bool {
        self.iter()
        .zip(rhs.iter())
        .all(|(l,r)|f(l,r))
    }

    fn is_elem_wise_smaller_eq(&self, rhs:&Self) -> bool {
       self.is_elem_wise_op(rhs, |l,r|l <= r)
    }

    fn is_elem_wise_strictly_smaller(&self,rhs:&Self) -> bool {
        self.is_elem_wise_op(rhs, |l,r|l < r)
    }

    fn is_elem_wise_larger_eq(&self, rhs:&Self) -> bool {
       self.is_elem_wise_op(rhs, |l,r|l >= r)
    }
 
    fn is_elem_wise_strictly_larger(&self,rhs:&Self) -> bool {
        self.is_elem_wise_op(rhs, |l,r|l > r)
    }

    fn len(&self) -> usize {
        self.iter()
            .cloned()
            .into_product()
    }

    fn elem_wise_binary(self, rhs:Self, f:impl Fn(usize,usize) -> usize) -> Self {
        Self::try_from_iter(
            self.into_iterator()
                .zip(rhs.into_iterator())
                .map(|(l,r)|f(l,r)))
            .ok().unwrap()
    }

    fn elem_wise_min(self,rhs:Self) -> Self {
        let f=|a,b|usize::min(a,b);
        Self::elem_wise_binary(self, rhs, f)
    }
    fn elem_wise_add(self,rhs:Self) -> Self {
        let f=|a,b|a+b;
        Self::elem_wise_binary(self, rhs, f)
    }
    fn elem_wise_mul(self,rhs:Self) -> Self {
        let f=|a,b|a*b;
        Self::elem_wise_binary(self, rhs, f)
    }
    fn elem_wise_mod(self,rhs:Self) -> Self {
        let f=|a,b|a % b;
        Self::elem_wise_binary(self, rhs, f)
    }

    fn try_elem_wise_binary(self, rhs:Self, f:impl Fn(usize,usize) -> Option<usize>) -> Option<Self> {
        self.into_iterator()
            .zip(rhs.into_iterator())
            .map(|(l,r)|f(l,r))
            .collect::<Option<Vec<usize>>>()
            .and_then(|ov|Self::try_from_iter(ov).ok())
    }

    fn try_elem_wise_sub(self, rhs:Self) -> Option<Self> {
        let f=|a,b|(a >= b).then(||a-b);
        Self::try_elem_wise_binary(self, rhs, f)
    }

}


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
        super::ContainerIndexIterator::new_exact_size(self.0)
            .map(|ind|Self(ind))
    }
}

impl Iter<usize> for usize {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a usize> where usize : 'a {
        std::iter::once(self)
    }
}

impl IntoIter<usize> for usize {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=usize> {
        std::iter::once(self)
    }
}
impl ContainerIndex for usize {
    fn index_iterator(self) -> impl ExactSizeIterator<Item=Self> {
        0..self
    }
}

impl TryFromIterator<usize,ContainerConstructError<usize>> for (usize,usize) {
    fn try_take_away<I:Iterator<Item=usize>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
        let first=iter.next().ok_or(LenTooSmallError::new(2,0))?;
        let second=iter.next().ok_or(LenTooSmallError::new(2,1))?;
        Ok((first,second))
    }
    crate::try_from_iter_impl!(usize);
}

impl<S:Into<[usize;N]>,const N:usize> From<S> for IndexN<N> {
    fn from(value: S) -> Self {
        IndexN(value.into()) // [value.0,value.1]
    }
}

impl Iter<usize> for (usize,usize) {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a usize> where usize : 'a {
        [&self.0,&self.1].into_iter()
    }
}

impl IntoIter<usize> for (usize,usize) {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=usize> {
        [self.0,self.1].into_iter()
    }
}

impl ContainerIndex for (usize,usize) {
    fn index_iterator(self) -> impl ExactSizeIterator<Item=Self> {
        IndexN::<2>::from(self)
            .index_iterator()
            .map(|s|s.0.into())
    }
}

pub fn row_major_index_iterator(size:(usize,usize)) -> impl ExactSizeIterator<Item=(usize,usize)> {
    size.index_iterator()
}

pub fn column_major_index_iterator(size:(usize,usize)) -> impl ExactSizeIterator<Item=(usize,usize)> {
    let flip=|(l,r)|(r,l);
    flip(size).index_iterator()
              .map(flip)
}

#[test]
fn test23() {
    let mut iter=(3,2).index_iterator();
    assert_eq!(iter.next(),Some((0,0)));
    assert_eq!(iter.next(),Some((0,1)));
    assert_eq!(iter.next(),Some((1,0)));
    assert_eq!(iter.next(),Some((1,1)));
    assert_eq!(iter.next(),Some((2,0)));
    assert_eq!(iter.next(),Some((2,1)));
    assert_eq!(iter.next(),None);
}