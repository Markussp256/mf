use super::IterMut;

pub trait IterMutIndexed<Index,T> {
    fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index,&'a mut T)> where T:'a;
}

pub fn iter_mut_indexed_impl<'a,T:'a>(s:&'a mut impl IterMut<T>) -> impl ExactSizeIterator<Item=(usize,&'a mut T)> {
    s.iter_mut()
     .enumerate()
}

impl<T> IterMutIndexed<usize,T> for Vec<T> {
    fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(usize,&'a mut T)> where T:'a {
        iter_mut_indexed_impl(self)
    }
}

impl<T,const N:usize> IterMutIndexed<usize,T> for [T;N] {
    fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(usize,&'a mut T)> where T : 'a {
        iter_mut_indexed_impl(self)
    }
}