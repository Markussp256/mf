pub trait IntoVec<T> {
    fn into_vec(self) -> Vec<T>;
}

impl<T> IntoVec<T> for Vec<T> {
    fn into_vec(self) -> Vec<T> {
        self
    }
}

impl<T,const N:usize> IntoVec<T> for [T;N] {
    fn into_vec(self) -> Vec<T> {
        self.into()
    }
}