use generic_array::{ArrayLength, GenericArray};


pub trait IntoVec<T> {
    fn into_vec(self) -> Vec<T>;
}

impl<T> IntoVec<T> for Vec<T> {
    fn into_vec(self) -> Vec<T> {
        self
    }
}

impl<T,N:ArrayLength> IntoVec<T> for GenericArray<T,N> {
    fn into_vec(self) -> Vec<T> {
        self.into_iter()
            .collect()
    }
}

impl<T,const N:usize> IntoVec<T> for [T;N] {
    fn into_vec(self) -> Vec<T> {
        self.into()
    }
}