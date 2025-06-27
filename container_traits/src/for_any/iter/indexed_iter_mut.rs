use super::IterMut;

pub trait IndexedIterMut<Index,T> {
    fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index,&'a mut T)> where T:'a;
}

pub fn indexed_iter_mut_impl<'a,T:'a>(s:&'a mut impl IterMut<T>) -> impl ExactSizeIterator<Item=(usize,&'a mut T)> {
    s.iter_mut()
     .enumerate()
}

impl<T> IndexedIterMut<usize,T> for Vec<T> {
    fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(usize,&'a mut T)> where T:'a {
        indexed_iter_mut_impl(self)
    }
}

impl<T,const N:usize> IndexedIterMut<usize,T> for [T;N] {
    fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(usize,&'a mut T)> where T : 'a {
        indexed_iter_mut_impl(self)
    }
}