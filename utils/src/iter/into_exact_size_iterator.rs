use std::iter::Take;

pub struct WithExactSize<I> {
    iter: I,
    len: usize,
}

impl<I> WithExactSize<I> {
    pub fn new(iter: I, len: usize) -> Self {
        WithExactSize { iter, len }
    }
}

impl<I> Iterator for WithExactSize<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.iter.next();
        if next_item.is_some() {
            self.len -= 1;
        }
        next_item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len;
        (len, Some(len))
    }

}


impl<I:Iterator> ExactSizeIterator for WithExactSize<I> {
    // not necessary because there is default impl based on size_hint
    // fn len(&self) -> usize {
    //     self.len
    // }
}

pub trait IntoExactSizeIterator : Sized+IntoIterator {
    fn into_exact_size_iter(self,len:usize) -> WithExactSize<Take<Self::IntoIter>> {
        WithExactSize::new(self.into_iter().take(len),len)
    }
}
impl<I:Sized+IntoIterator> IntoExactSizeIterator for I {}