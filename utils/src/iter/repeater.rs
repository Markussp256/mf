// problem std::iter::repeat_with(f).take(n) does not implement ExactSizeIterator nor clone

#[derive(Debug)]
pub struct RepeaterN<T, F:Fn() -> T> {
    f: F,
    len: usize,
}

impl<T,F:Fn() -> T> RepeaterN<T,F> {
    pub fn new(f: F, len: usize) -> Self {
        RepeaterN {
            f,
            len,
        }
    }
}

// using derive(Clone) would require T:Clone
impl<T,F:Clone+Fn()->T> Clone for RepeaterN<T,F> {
    fn clone(&self) -> Self {
        Self::new(self.f.clone(),self.len.clone())
    }
}

impl<T, F:Fn() -> T> Iterator for RepeaterN<T,F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.len > 0).then(||
            {
                self.len -= 1;
                (self.f)()
            }
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len;
        (len, Some(len))
    }
}

impl<T,F:Fn()-> T> ExactSizeIterator for RepeaterN<T,F> {}