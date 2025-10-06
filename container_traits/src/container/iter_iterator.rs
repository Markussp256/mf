use std::marker::PhantomData;

use crate::{index_iterator::ContainerIndexIterator, AsSlice, ContainerSize, Size};

#[derive(Debug)]
pub struct IterIterator<'a,Index,T> {
    ptr: * const T,
    cii:ContainerIndexIterator<Index>,
     _marker: PhantomData<&'a  T>,
}

impl<'a,Index:ContainerSize,T> IterIterator<'a,Index,T> {
    pub fn new<C:Size<Index>+AsSlice<T>>(c: &'a  C, cii:ContainerIndexIterator<Index>) -> Self {
        let linear_index=cii.linear_index();
        let ptr=c.as_slice().as_ptr().wrapping_add(linear_index);
        Self{ptr,cii,_marker:PhantomData}
    }

    pub fn index(&self) -> &Index {
        self.cii
            .current()
    }
}

impl<'a,Index : ContainerSize,T> ExactSizeIterator for IterIterator<'a,Index,T> {}

impl<'a,Index:ContainerSize,T> Iterator for IterIterator<'a,Index,T> where T : 'a {
    type Item = &'a  T;

    fn next(& mut self) -> Option<Self::Item> {
        let ls=self.cii
            .next_linear_index_step()?.1;
        self.ptr=self.ptr.wrapping_add(ls);
        unsafe { self.ptr.as_ref() }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cii
            .size_hint()
    }
}
