use generic_array::{ArrayLength, GenericArray};


pub trait Reverse {
    fn reverse(self) -> Self; 
}

macro_rules! impl_rev {
    () => {
        fn reverse(mut self) -> Self {
            self.as_mut_slice()
                .reverse();
            self
        }
    };
}

impl<T> Reverse for Vec<T> {
    impl_rev!();
}

impl<T, N : ArrayLength> Reverse for GenericArray<T,N> {
    impl_rev!();
}

impl<T,const N:usize> Reverse for [T;N] {
    impl_rev!();
}