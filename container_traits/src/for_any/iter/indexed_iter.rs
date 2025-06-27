use crate::{ContainerIndex, Get};

pub trait IndexedIter<Index,T> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T : 'a;
}


impl<T> IndexedIter<usize,T> for Vec<T> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(usize,&'a T)> where T : 'a {
        self.iter().enumerate()
    }
}

impl<T,const N:usize> IndexedIter<usize,T> for [T;N] {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(usize,&'a T)> where T : 'a {
        self.iter().enumerate()
    }
}

pub fn impl_indexed_iter_from_get
    <'a,
     Index : ContainerIndex,
     T : 'a,
     G : 'a+Get<Index,T>>(g:&'a G,size:Index) -> impl ExactSizeIterator<Item=(Index,&'a T)> {
        size.index_iterator()
        .map(move |index:Index|(index.clone(), g.get(index).unwrap()))
}