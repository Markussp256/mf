pub trait TryIntoElement<Index,T> : Sized {
    fn try_into_element(self,index:Index) -> Option<T>;
}

macro_rules! impl_try_into_element {
    () => {
        fn try_into_element(self,index:usize) -> Option<T> {
            self.into_iter()
                .nth(index)       
        }
    }
}

impl<T> TryIntoElement<usize,T> for Vec<T> {
    impl_try_into_element!();
}

impl<T,const N:usize> TryIntoElement<usize,T> for [T;N] {
    impl_try_into_element!();
}