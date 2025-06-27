
pub trait IterMut<T> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a;
}

macro_rules! impl_iter_mut {
    () => {
        fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
            self.as_mut_slice()
                .iter_mut()
        }
    };
}

impl<T> IterMut<T> for Vec<T> {
    impl_iter_mut!();
}

impl<T, const N:usize> IterMut<T> for  [T;N] {
    impl_iter_mut!();
}