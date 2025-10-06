use crate::{index_iterator::ContainerIndexIterator, ContainerSize, Get};

pub trait IterIndexed<Index,T> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T : 'a;
}


impl<T> IterIndexed<usize,T> for Vec<T> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(usize,&'a T)> where T : 'a {
        self.iter().enumerate()
    }
}

impl<T,const N:usize> IterIndexed<usize,T> for [T;N] {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(usize,&'a T)> where T : 'a {
        self.iter().enumerate()
    }
}

pub fn impl_iter_indexed_from_get
    <'a,
     Index : ContainerSize,
     T : 'a,
     G : 'a+Get<Index,T>>(g:&'a G,size:Index) -> impl ExactSizeIterator<Item=(Index,&'a T)> {
        ContainerIndexIterator::new_exact_size(size)
            .map(move |index:Index|(index.clone(), g.get(index).unwrap()))
}