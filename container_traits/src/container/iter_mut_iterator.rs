use std::marker::PhantomData;

use crate::{index_iterator::ContainerIndexIterator, AsMutSlice, ContainerSize, Size};

#[derive(Debug)]
pub struct IterMutIterator<'a,Index,T> {
    ptr: *mut T,
    cii:ContainerIndexIterator<Index>,
     _marker: PhantomData<&'a mut T>,
}

impl<'a,Index:ContainerSize, T> IterMutIterator<'a,Index, T> {
    pub fn new<C:Size<Index>+AsMutSlice<T>>(c: &'a mut C, cii:ContainerIndexIterator<Index>) -> Self {
        let linear_index=cii.linear_index();
        let ptr=c.as_mut_slice().as_mut_ptr().wrapping_add(linear_index);
        Self{ptr,cii,_marker:PhantomData}
    }
}

impl<'a,Index:ContainerSize, T> Iterator for IterMutIterator<'a,Index,T> where T : 'a {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let ls=self.cii
            .next_linear_index_step()?.1;
        self.ptr=self.ptr.wrapping_add(ls);
        unsafe { self.ptr.as_mut() }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cii
            .size_hint()
    }
}

impl<'a,Index : ContainerSize, T> ExactSizeIterator for IterMutIterator<'a,Index, T> {}
