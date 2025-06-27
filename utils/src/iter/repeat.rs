// problem std::iter::repeat(t).take(n) does not implement ExactSizeIterator nor clone

#[derive(Clone)]
pub struct RepeatN<T> {
    value: T,
    len: usize,
}

impl<T> RepeatN<T> {
    pub fn new(value: T, len: usize) -> Self {
        RepeatN {
            value,
            len,
        }
    }
}

impl<T: Clone> Iterator for RepeatN<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.len > 0).then(||
            {
                self.len -= 1;
                self.value.clone()
            }
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len;
        (len, Some(len))
    }
}

impl<T:Clone> ExactSizeIterator for RepeatN<T> {
    // would not be necessary
    // fn len(&self) -> usize {
    //     self.len
    // }
}