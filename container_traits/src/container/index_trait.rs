
use std::fmt::Debug;
use crate::{index_type::IndexN, ContainerConstructError, IndexOutOfBoundsError, IntoIter, IntoProduct, Iter, IterMut, LenTooSmallError, TryFromIterator};

pub trait ContainerIndex
    : Debug
     +Default
     +Clone
     +PartialOrd
     +Ord
     +Iter<usize>
     +IterMut<usize>
     +IntoIter<usize>
     +TryFromIterator<usize,ContainerConstructError<usize>> {

    const DIM:usize;

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

pub trait ContainerSize : ContainerIndex {

    // number of different indizes for the given size
    fn numel(&self) -> usize {
        self.iter()
            .cloned()
            .into_product()
    }

    // linear index for ptr
    fn linear_index(&self,c:Self) -> Result<usize,IndexOutOfBoundsError<Self>> {
        IndexOutOfBoundsError::try_new(self, &c)?;
        let css:Vec<(usize,usize)>=c.into_iterator().zip(self.iter().cloned()).collect();
        let mut current=css.last().unwrap().0;
        for (ci,si) in css.into_iter().rev().skip(1) {
            current *= si;
            current += ci;
        }
        Ok(current)
    }
}


impl Iter<usize> for usize {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a usize> where usize : 'a {
        std::iter::once(self)
    }
}

impl IterMut<usize> for usize {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut usize> where usize : 'a {
        std::iter::once(self)
    }
}

impl IntoIter<usize> for usize {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=usize> {
        std::iter::once(self)
    }
}

impl TryFromIterator<usize,ContainerConstructError<usize>> for (usize,usize) {
    fn try_take_away<I:Iterator<Item=usize>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
        let first=iter.next().ok_or(LenTooSmallError::new(2,0))?;
        let second=iter.next().ok_or(LenTooSmallError::new(2,1))?;
        Ok((first,second))
    }
}

impl Iter<usize> for (usize,usize) {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a usize> where usize : 'a {
        [&self.0,&self.1].into_iter()
    }
}

impl IterMut<usize> for (usize,usize) {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut usize> where usize : 'a {
        [&mut self.0,& mut self.1].into_iter()
    }
}

impl IntoIter<usize> for (usize,usize) {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=usize> {
        [self.0,self.1].into_iter()
    }
}

impl ContainerIndex for usize {
    const DIM:usize = 1;
}

impl ContainerSize for usize {}

impl ContainerIndex for (usize,usize) {
    const DIM:usize=2;
}

impl ContainerSize for (usize,usize) {}

impl<const N:usize> ContainerIndex for IndexN<N> {
    const DIM:usize=N;
}

impl<const N:usize> ContainerSize for IndexN<N> {}
